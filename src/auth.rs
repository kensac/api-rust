use std::{sync::Arc, vec};

use axum::{http::response, routing::post, Json, Router};
use axum_extra::extract::cookie;
use hyper::{client, StatusCode};
use reqwest::{cookie::{CookieStore, Jar}, Url};
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

    let appwrite_id = std::env::var("APPWRITE_ID").unwrap();

    let log_in_url = "https://cloud.appwrite.io/v1/account/sessions";

    let response = client
        .post(log_in_url)
        .header("X-Appwrite-Project", appwrite_id)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .unwrap();

    let cookies_vec: Vec<String> = response
        .headers()
        .get_all("Set-Cookie")
        .into_iter()
        .filter_map(|value| value.to_str().ok().map(String::from))
        .collect();

    // Check if any cookies were found and print them
    if cookies_vec.is_empty() {
        println!("No Set-Cookie headers found");
    } else {
        for cookie in cookies_vec {
            cookies.add(Cookie::parse(cookie.clone()).unwrap());
            let cookie = cookie.clone().replace("domain", "localhost");
            cookies.add(Cookie::parse(cookie.clone()).unwrap());
        }
    }

    let response_status = response.status();
    if response_status.is_success() {
        Ok(StatusCode::CREATED)
    } else {
        let response_body = response.text().await.unwrap();
        Err((response_status, response_body))
    }
}

#[axum::debug_handler]
pub async fn get_jwt(cookies: Cookies) -> Result<StatusCode, StatusCode> {

    let jar = Arc::new(Jar::default());

    let client = reqwest::Client::builder()
        .cookie_provider(Arc::clone(&jar))
        .cookie_store(true)
        .build()
        .unwrap();

    let appwrite_id = std::env::var("APPWRITE_ID").unwrap();

    let get_jwt_url = "https://cloud.appwrite.io/v1/account/jwt";

    let client = client.post(get_jwt_url);

    for cookie in cookies.list().iter() {
        jar.add_cookie_str(
            cookie.to_string().as_str(),
            &reqwest::Url::parse("https://cloud.appwrite.io").unwrap(),
        );
    }

    println!("{:?}", jar.cookies(&Url::parse("https://cloud.appwrite.io").unwrap()));

    let response = client
        .header("X-Appwrite-Project", appwrite_id)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap();

    let response_status = response.status();
    if response_status.is_success() {
        let response_body = response.text().await.unwrap();
        println!("{}", response_body);
        Ok(StatusCode::CREATED)
    } else {
        let response_body = response.text().await.unwrap();
        println!("{}", response_body);
        Err(response_status)
    }
}

pub async fn get_auth_routes() -> Router {
    Router::new()
        .route("/login", post(log_in_with_email_and_password))
        .route("/jwt", post(get_jwt))
}