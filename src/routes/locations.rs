use axum::{
    extract::{Path, State},
    response::Response,
    routing::get,
    Json, Router,
};
use hyper::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    prisma::{self, location::Data},
    base_types::AppState,
};

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateLocationEntity {
    name: String,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/location",
    responses(
        (status = 200, description = "Create a new location", body = String),
        (status = 400, description = "Bad request")
    ),
    request_body = CreateLocationEntity
)]
async fn create_location(
    State(app_state): State<AppState>,
    Json(body): Json<CreateLocationEntity>,
) -> Result<Response<String>, StatusCode> {
    match app_state
        .client
        .location()
        .create(body.name, vec![])
        .exec()
        .await
    {
        Ok(_location) => Ok(Response::new("Created location succesfully".to_string())),
        Err(_err) => Err(StatusCode::BAD_REQUEST),
    }
}
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/locations",
    responses(
        (status = 200, description = "Get all locations"),
        (status = 400, description = "Bad request")
    )
)]
async fn get_all_locations(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Data>>, StatusCode> {
    match app_state.client.location().find_many(vec![]).exec().await {
        Ok(locations) => Ok(Json(locations)),
        Err(_err) => Err(StatusCode::BAD_REQUEST),
    }
}
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/locations/{id}",
    responses(
        (status = 200, description = "Get a location by id"),
        (status = 400, description = "Bad request")
    )
)]
async fn get_location_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Data>, (StatusCode, String)> {
    match app_state
        .client
        .location()
        .find_unique(prisma::location::UniqueWhereParam::IdEquals(id))
        .exec()
        .await
    {
        Ok(location) => match location {
            Some(location) => Ok(Json(location)),
            None => Err((StatusCode::NOT_FOUND, "".to_string())),
        },
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}
#[axum::debug_handler]
#[utoipa::path(
    delete,
    path = "/locations/{id}",
    responses(
        (status = 200, description = "Delete a location by id"),
        (status = 400, description = "Bad request")
    )
)]
async fn delete_location_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response<String>, StatusCode> {
    match app_state
        .client
        .location()
        .delete(prisma::location::UniqueWhereParam::IdEquals(id))
        .exec()
        .await
    {
        Ok(_) => Ok(Response::new("Deleted location succesfully".to_string())),
        Err(_err) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn location_get_router() -> Router {
    let state = AppState::new().await;
    Router::new()
        .route("/", get(get_all_locations).post(create_location))
        .route(
            "/{id}",
            get(get_location_by_id).delete(delete_location_by_id),
        )
        .with_state(state)
}
