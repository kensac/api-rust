use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::http::HeaderValue;
use axum::routing::get;
use axum::{BoxError, Router};
use docs::ApiDoc;
use hyper::{Method, StatusCode};
use tower::buffer::BufferLayer;
use tower::limit::RateLimitLayer;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

use crate::base_types::AppState;
use crate::{docs, routes, utils};

pub async fn new_app(app_state: AppState) -> Router {
    let service_layer = new_service_layer();
    let cors_layer = new_cors_layer();

    let sponsor_routes = routes::sponsors::sponsor_get_router(app_state.clone());
    let hackathon_routes = routes::hackathons::hackathon_get_router(app_state.clone());
    let extra_credit_class_routes =
        routes::extra_credit_classes::extra_credit_class_get_router(app_state.clone());
    let location_routes = routes::locations::location_get_router(app_state.clone());
    let event_routes = routes::events::events_get_router(app_state.clone());
    let scans_routes = routes::scans::scans_get_router(app_state.clone());
    let user_routes = routes::users::user_get_router(app_state.clone());

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
        .nest("/users", user_routes)
        .merge(Redoc::with_url("/docs", ApiDoc::openapi()))
        .merge(service_layer)
        .merge(cors_layer)
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
                    format!("Unhandled error: {err}"),
                )
            }))
            .layer(BufferLayer::new(1024))
            .layer(RateLimitLayer::new(100, Duration::from_secs(5))),
    )
}

fn new_cors_layer() -> Router {
    Router::new().layer(
        CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::DELETE])
            .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
            .allow_credentials(true),
    )
}
