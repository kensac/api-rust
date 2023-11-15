pub mod database;
pub mod docs;
#[allow(warnings)]
pub mod prisma;
pub mod routes;
pub mod utils;

use std::net::SocketAddr;
use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::routing::get;
use axum::{ BoxError, Json, Router };
use docs::ApiDoc;
use hyper::StatusCode;
use tower::buffer::BufferLayer;
use tower::limit::RateLimitLayer;
use tower::ServiceBuilder;

use utils::get_port;
use utoipa::OpenApi;
use utoipa_redoc::{ Redoc, Servable };

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let port = get_port();

    let app = new_app().await;
    let address = SocketAddr::from(([0, 0, 0, 0], port));

    axum::Server::bind(&address).serve(app.into_make_service()).await.unwrap();
}

async fn new_app() -> Router {
    let sponsor_routes = routes::sponsors::sponsor_get_router().await;
    let hackathon_routes = routes::hackathons::hackathon_get_router().await;
    let extra_credit_class_routes =
        routes::extra_credit_classes::extra_credit_class_get_router().await;
    let location_routes = routes::locations::location_get_router().await;
    let event_routes = routes::events::events_get_router().await;

    Router::new()
        .route("/", get(utils::hello_world))
        .route("/health", get(utils::health_check))
        .nest("/sponsors", sponsor_routes)
        .nest("/hackathons", hackathon_routes)
        .nest("/extra_credit/classes", extra_credit_class_routes)
        .nest("/locations", location_routes)
        .nest("/events", event_routes)
        .merge(Redoc::with_url("/docs", ApiDoc::openapi()))
        /*         .route("/test", get(server_side_auth)) */
        .layer(
            ServiceBuilder::new()
                .layer(
                    HandleErrorLayer::new(|err: BoxError| async move {
                        (StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled error: {}", err))
                    })
                )
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(100, Duration::from_secs(5)))
        )
        .fallback(get(utils::handle_404))
}
