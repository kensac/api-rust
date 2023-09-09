use axum::{Router, routing::MethodRouter};

pub fn route(path: &str, method_router: MethodRouter) -> Router  {
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