#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(
    clippy::single_call_fn,
    clippy::exhaustive_structs,
    clippy::implicit_return
)]
#![allow(
    clippy::unwrap_used,
    clippy::exit,
    clippy::str_to_string,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::shadow_reuse,
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
    clippy::mod_module_files,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::cargo_common_metadata
)]

pub mod app;
pub mod auth_guard;
pub mod base_types;
pub mod database;
pub mod docs;
#[allow(warnings)]
pub mod prisma;
pub mod routes;
pub mod upload_service;
pub mod utils;

use std::net::SocketAddr;

use app::new_app;

use base_types::AppState;
use tokio::net::TcpListener;
use utils::get_port;

#[macro_use]
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
