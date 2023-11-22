use axum::{routing::post, Json, Router};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Serialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LogInWithEmailStruct {
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
    #[serde(default)]
    return_secure_token: Option<bool>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LogInWithEmailStructResponse {
    kind: String,
    local_id: String,
    email: String,
    display_name: String,
    id_token: String,
    registered: bool,
    refresh_token: String,
    expires_in: String,
}
#[axum::debug_handler]
pub async fn log_in_with_email_and_password(
    cookies: Cookies,
    Json(body): Json<LogInWithEmailStruct>,
) -> Result<StatusCode, (StatusCode, String)> {
    let client = reqwest::Client::new();
    let firebase_api_key = std::env::var("FIREBASE_API_KEY").unwrap();
    let log_in_url = "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key="
        .to_string()
        + &firebase_api_key;

    let response = client.post(log_in_url).json(&body).send().await.unwrap();
    let response_status = response.status();
    if response_status.is_success() {
        let response_body: LogInWithEmailStructResponse = response.json().await.unwrap();
        let mut cookie = Cookie::build("JWT", response_body.id_token)
            .path("/")
            .finish();
        cookie.set_http_only(true);
        cookie.set_secure(true);
        cookies.add(cookie);
        Ok(StatusCode::OK)
    } else {
        let response_body = response.text().await.unwrap();
        Err((response_status, response_body))
    }
}

pub async fn get_auth_routes() -> Router {
    Router::new().route("/login", post(log_in_with_email_and_password))
}
