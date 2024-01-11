use std::time::Duration;

use axum::{error_handling::HandleErrorLayer, http::HeaderValue, routing::get, BoxError, Router};
use hyper::{Method, StatusCode};
use socketioxide::SocketIo;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

use crate::{
    base_types::{AppState, APP_STATE},
    docs::ApiDoc,
    email_service::send_test_email,
    routes,
    socket::on_connect,
    utils,
};

pub async fn new_app() -> Router {
    let service_layer = create_service_layer();
    let cors_layer = create_cors_layer();

    let (socket_layer, io) = SocketIo::new_layer();
    io.ns("/socket", on_connect);

    let app_state = AppState::new(io).await;
    APP_STATE.set(app_state.clone()).unwrap();

    Router::new()
        .route("/email", get(send_test_email))
        .with_state(app_state.clone())
        .route("/shutdown", get(utils::shutdown))
        .route("/", get(utils::hello_world))
        .route("/health", get(utils::health_check))
        .merge(setup_routes(app_state))
        .merge(Redoc::with_url("/docs", ApiDoc::openapi()))
        .merge(service_layer)
        .merge(cors_layer)
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new())
        .fallback(get(utils::handle_404))
        .layer(socket_layer)
}

fn setup_routes(app_state: AppState) -> Router {
    Router::new()
        .nest(
            "/sponsors",
            routes::sponsors::sponsor_get_router(app_state.clone()),
        )
        .nest(
            "/hackathons",
            routes::hackathons::hackathon_get_router(app_state.clone()),
        )
        .nest(
            "/extra_credit/classes",
            routes::extra_credit_classes::extra_credit_class_get_router(app_state.clone()),
        )
        .nest(
            "/locations",
            routes::locations::location_get_router(app_state.clone()),
        )
        .nest(
            "/events",
            routes::events::events_get_router(app_state.clone()),
        )
        .nest("/scans", routes::scans::scans_get_router(app_state.clone()))
        .nest("/users", routes::users::user_get_router(app_state))
}

fn create_service_layer() -> Router {
    Router::new().layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|err: BoxError| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {err}"),
                )
            }))
            .layer(BufferLayer::new(1024))
            .layer(RateLimitLayer::new(100, Duration::from_secs(5))),
    )
}

fn create_cors_layer() -> Router {
    Router::new().layer(
        CorsLayer::new()
            .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
            .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
            .allow_credentials(true),
    )
}
