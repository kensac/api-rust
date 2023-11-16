use axum::{ http::status, routing::MethodRouter, Router, Json };
use hyper::StatusCode;
use regex::Regex;

use crate::prisma::PrismaClient;

pub fn route(path: &str, method_router: MethodRouter) -> Router {
    return Router::new().route(path, method_router);
}

#[axum::debug_handler]
pub async fn handle_404() -> (StatusCode, &'static str) {
    return (StatusCode::INTERNAL_SERVER_ERROR, "The requested resource was not found.");
}

pub async fn hello_world() -> &'static str {
    "Hello, World!"
}

pub fn get_port() -> u16 {
    let port = std::env
        ::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    return port;
}

#[utoipa::path(get, path = "/health", responses((status = 200, description = "Service is Alive")))]
pub async fn health_check() -> status::StatusCode {
    return status::StatusCode::OK;
}

#[derive(Clone)]
pub struct AppState {
    pub client: PrismaClient,
}

pub async fn get_app_state() -> AppState {
    let client = PrismaClient::_builder().build().await.expect("Didn't connect to database");

    let state = AppState { client: client };
    return state;
}

lazy_static! {
    pub static ref UUID_VALIDATOR: Regex = Regex::new(r"[a-z]{2}$").unwrap();
}

#[axum::debug_handler]
pub async fn server_side_auth() -> Json<()> {
    dotenv::dotenv().ok();

    let appwrite_secret = std::env::var("APPWRITE_SECRET").unwrap();
    let appwrite_id = std::env::var("APPWRITE_ID").unwrap();

    let client = reqwest::Client::new();

    let url = format!(
        "https://cloud.appwrite.io/v1/databases/{}/collections/{}/documents",
        "test",
        "sample"
    );
    let response = client
        .get(&url)
        .header("X-Appwrite-Project", appwrite_id)
        .header("X-Appwrite-Key", appwrite_secret)
        .send().await;

    println!("{:?}", response.unwrap().text().await.unwrap());
    Json(())
}
