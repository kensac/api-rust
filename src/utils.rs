use std::process::exit;

use axum::http::status;
use hyper::StatusCode;
use regex::Regex;

#[axum::debug_handler]
pub async fn handle_404() -> (StatusCode, &'static str) {
    (
        StatusCode::NOT_FOUND,
        "We couldn't find the resource you requested.",
    )
}

pub async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[utoipa::path(get, path = "/health", responses((status = 200, description = "Service is Alive")))]
pub async fn health_check() -> status::StatusCode {
    status::StatusCode::OK
}

pub async fn shutdown() {
    exit(0);
}

pub fn get_port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000)
}

lazy_static! {
    pub static ref UUID_VALIDATOR: Regex = Regex::new(r"[a-z]{2}$").unwrap();
}
