pub mod app;
pub mod auth;
pub mod base_types;
pub mod database;
pub mod docs;
#[allow(warnings)]
pub mod prisma;
pub mod routes;
pub mod utils;
pub mod  auth_guard;

use std::net::SocketAddr;

use app::new_app;

use utils::get_port;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        .init();

    dotenv::dotenv().ok();

    let port = get_port();

    let app = new_app().await;
    let address = SocketAddr::from(([0, 0, 0, 0], port));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
