use axum::{http::status, routing::MethodRouter, Router};
use hyper::StatusCode;
use regex::Regex;

use crate::prisma::PrismaClient;

fn _route(path: &str, method_router: MethodRouter) -> Router {
    Router::new().route(path, method_router)
}

#[axum::debug_handler]
pub async fn handle_404() -> (StatusCode, &'static str) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "The requested resource was not found.",
    )
}

pub async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[utoipa::path(get, path = "/health", responses((status = 200, description = "Service is Alive")))]
pub async fn health_check() -> status::StatusCode {
    status::StatusCode::OK
}

pub fn get_port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000)
}

#[derive(Clone)]
pub struct AppState {
    pub client: PrismaClient,
}

pub async fn get_app_state() -> AppState {
    let client = PrismaClient::_builder()
        .build()
        .await
        .expect("Didn't connect to database");

    AppState { client }
}

lazy_static! {
    pub static ref UUID_VALIDATOR: Regex = Regex::new(r"[a-z]{2}$").unwrap();
}