use axum::extract::State;
use gcp_auth::{AuthenticationManager, CustomServiceAccount};
use hyper::body::Bytes;
use urlencoding::encode;

use crate::base_types::{AppState, CreateResponse};

struct Buckets;

impl Buckets {
    pub const RESUME: &'static str = "hackpsu_api_rust_resumes";
}

#[derive(Debug)]
pub struct StorageBucketService {
    pub client: reqwest::Client,
}

impl StorageBucketService {
    pub fn new() -> Self {
        let path = std::path::Path::new("./google-service-account.json");
        // I left this in so that we can cause a runtime error if the file is not found. This makes sure that none of the services fail silently.
        let _service_account = CustomServiceAccount::from_file(path).unwrap();
        let client = reqwest::Client::new();
        StorageBucketService { client }
    }

    pub async fn create_jwt() -> String {
        let path = std::path::Path::new("./google-service-account.json");
        let service_account = CustomServiceAccount::from_file(path).unwrap();
        let authentication_manager = AuthenticationManager::from(service_account);
        let scopes = &["https://www.googleapis.com/auth/devstorage.full_control"];
        let token = authentication_manager.get_token(scopes).await.unwrap();
        token.as_str().to_string()
    }
}

pub async fn upload_file(
    file: Vec<u8>,
    bucket_name: &str,
    folder: Option<&str>,
    file_name: &str,
    app_state: AppState,
) -> CreateResponse {
    let jwt = StorageBucketService::create_jwt().await;

    let url = format!(
        "https://storage.googleapis.com/upload/storage/v1/b/{}/o",
        bucket_name
    );

    let name = match folder {
        Some(folder) => format!("{}/{}", folder, file_name),
        None => file_name.to_string(),
    };

    let res = app_state
        .storage_bucket_service
        .client
        .post(&url)
        .header("Authorization", format!("Bearer {}", jwt))
        .header("Content-Length", file.len())
        .query(&[("uploadType", "media"), ("name", &name)])
        .body(file)
        .send()
        .await
        .unwrap();

    let status_code = res.status();

    if status_code.is_success() {
        Ok((hyper::StatusCode::NO_CONTENT, ()))
    } else {
        Err((
            hyper::StatusCode::INTERNAL_SERVER_ERROR,
            res.text()
                .await
                .unwrap_or("Failed to Upload File and convert error to string.".to_string()),
        ))
    }
}

pub async fn download_file(
    bucket_name: &str,
    folder: Option<&str>,
    file_name: &str,
    app_state: AppState,
) -> Result<Bytes, (String, hyper::StatusCode)> {
    let jwt = StorageBucketService::create_jwt().await;

    let name = match folder {
        Some(folder) => format!("{}/{}", folder, file_name),
        None => file_name.to_string(),
    };

    let url = format!(
        "https://storage.googleapis.com/storage/v1/b/{}/o/{}",
        bucket_name,
        encode(&name)
    );

    let res = app_state
        .storage_bucket_service
        .client
        .get(&url)
        .header("Authorization", format!("Bearer {}", jwt))
        .query(&[("alt", "media")])
        .send()
        .await
        .unwrap();

    let status_code = res.status();

    if status_code.is_success() {
        Ok(res.bytes().await.unwrap())
    } else {
        Err((
            res.text()
                .await
                .unwrap_or("Failed to Download File and convert error to string.".to_string()),
            hyper::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

pub async fn test_file(State(app_state): State<AppState>) -> CreateResponse {
    match download_file(Buckets::RESUME, Some("test"), "test.txt", app_state).await {
        Ok(file) => {
            println!("File: {:?}", file);
            std::fs::write("test2.txt", file).unwrap();
            Ok((hyper::StatusCode::OK, ()))
        }
        Err((err, status_code)) => Err((status_code, err)),
    }
}
