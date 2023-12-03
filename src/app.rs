use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::routing::get;
use axum::{BoxError, Router};
use docs::ApiDoc;
use hyper::StatusCode;
use tower::buffer::BufferLayer;
use tower::limit::RateLimitLayer;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;

use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

use crate::prisma::organizer;
use crate::{docs, routes, utils};

pub async fn new_app() -> Router {
    let service_layer = new_service_layer();

    let sponsor_routes = routes::sponsors::sponsor_get_router().await;
    let hackathon_routes = routes::hackathons::hackathon_get_router().await;
    let extra_credit_class_routes =
        routes::extra_credit_classes::extra_credit_class_get_router().await;
    let location_routes = routes::locations::location_get_router().await;
    let event_routes = routes::events::events_get_router().await;
    let scans_routes = routes::scans::scans_get_router().await;
    let organizer_routes = routes::organizers::routes().await;

    Router::new()
        .route("/shutdown", get(utils::shutdown))
        .route("/", get(utils::hello_world))
        .route("/health", get(utils::health_check))
        .nest("/sponsors", sponsor_routes)
        .nest("/hackathons", hackathon_routes)
        .nest("/extra_credit/classes", extra_credit_class_routes)
        .nest("/locations", location_routes)
        .nest("/events", event_routes)
        .nest("/scans", scans_routes)
        .nest("/organizers", organizer_routes)
        .merge(Redoc::with_url("/docs", ApiDoc::openapi()))
        .merge(service_layer)
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new())
        .fallback(get(utils::handle_404))
}

fn new_service_layer() -> Router {
    Router::new().layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|err: BoxError| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled error: {}", err),
                )
            }))
            .layer(BufferLayer::new(1024))
            .layer(RateLimitLayer::new(100, Duration::from_secs(5))),
    )
}
