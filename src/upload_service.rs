use axum::extract::State;
use gcp_auth::{AuthenticationManager, CustomServiceAccount};
use hyper::body::Bytes;

use crate::base_types::{AppState, CreateResponse, GetResponse};

#[derive(Debug)]
pub struct UploadService {
    client: reqwest::Client,
}

impl UploadService {
    pub fn new() -> Self {
        let path = std::path::Path::new("./google-service-account.json");
        let service_account = CustomServiceAccount::from_file(path).unwrap();
        let client = reqwest::Client::new();
        UploadService { client }
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

pub async fn upload_file(file: Vec<u8>, file_name: String, app_state: AppState) -> CreateResponse {
    let jwt = UploadService::create_jwt().await;
    let bucket_name = "hackpsu_api_rust_resumes";
    let url = format!(
        "https://storage.googleapis.com/upload/storage/v1/b/{}/o",
        bucket_name
    );
    let res = app_state
        .upload_service
        .client
        .post(&url)
        .header("Authorization", format!("Bearer {}", jwt))
        .header("Content-Length", file.len())
        .query(&[("uploadType", "media"), ("name", &file_name)])
        .body(file)
        .send()
        .await
        .unwrap();
    let status_code = res.status();
    println!("{:?}", status_code);
    if status_code.is_success() {
        Ok((hyper::StatusCode::NO_CONTENT, ()))
    } else {
        Err((
            hyper::StatusCode::INTERNAL_SERVER_ERROR,
            "Error uploading file".to_string(),
        ))
    }
}

pub async fn test_file(State(app_state): State<AppState>) -> CreateResponse {
    let file = std::fs::read("test.txt").unwrap();
    upload_file(file, "test.txt".to_string(), app_state).await
}
