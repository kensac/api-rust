use axum::{http::status, routing::MethodRouter, Router};

use crate::prisma::PrismaClient;

pub fn route(path: &str, method_router: MethodRouter) -> Router {
    return Router::new().route(path, method_router);
}

pub async fn handle_404() -> &'static str {
    return "404";
}

pub async fn hello_world() -> &'static str {
    "Hello, World!"
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
    let client = PrismaClient::_builder()
        .build()
        .await
        .expect("Didn't connect to database");

    let state = AppState { client: client };
    return state;
}
