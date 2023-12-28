pub mod app;
pub mod auth_guard;
pub mod base_types;
pub mod database;
pub mod docs;
#[allow(warnings)]
pub mod prisma;
pub mod routes;
pub mod socket;
pub mod upload_service;
pub mod utils;

use std::net::SocketAddr;

use app::new_app;

use base_types::AppState;
use tokio::net::TcpListener;
use utils::get_port;

extern crate lazy_static;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    dotenv::dotenv().ok();

    let port = get_port();

    let app_state = AppState::new().await;

    let app = new_app(app_state).await;
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
