use axum::{routing::MethodRouter, Router};

use crate::prisma::PrismaClient;

#[derive(Clone)]
pub struct AppState {
    pub client: PrismaClient,
}

pub fn route(path: &str, method_router: MethodRouter) -> Router {
    return Router::new().route(path, method_router);
}
#[axum::debug_handler]
pub async fn handler_404() -> &'static str {
    return "404";
}
#[axum::debug_handler]
pub async fn hello_world() -> &'static str {
    "Hello, World!"
}

pub async fn get_app_state() -> AppState {
    let client = PrismaClient::_builder()
        .build()
        .await
        .expect("Didn't connect to database");

    let state = AppState { client: client };
    return state;
}
