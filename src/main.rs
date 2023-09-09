#[allow(warnings)]
pub mod prisma;
pub mod database;
pub mod utils;
pub mod routes;

use axum::Router;
use axum::routing::{get, post};

use prisma::PrismaClient;
use prisma_client_rust::NewClientError;
use routes::user_routes::{create_user_route, get_first_user_route};

#[derive(Clone)]
pub struct AppState {
    client: PrismaClient,
}

#[tokio::main]
async fn main () {
    let client = PrismaClient::_builder().build().await.expect("Didn't connect to database");
    let state = AppState { client: client };
    let app = Router::new()
    .route("/users/", post(create_user_route).get(get_first_user_route))
    .route("/", get(utils::hello_world))
    .with_state(state)
    .fallback(get(utils::handler_404));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
