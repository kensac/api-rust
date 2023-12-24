use axum::{
    extract::{Path, State},
    middleware,
    routing::get,
    Extension, Json, Router,
};
use hyper::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    auth_guard::{self, permission_check},
    base_types::{AppState, CreateResponse, DeleteResponse, GetResponse},
    prisma::{self, location::Data, Role},
};

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LocationEntity {
    _id: String,
    _name: String,
}

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
        (status = 201, description = "Created a new location"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    ),
    request_body = CreateLocationEntity,
    security(
        ("http" = ["Exec", "Tech", "Team"])
    )
)]
async fn create_location(
    State(app_state): State<AppState>,
    Extension(request_user): Extension<auth_guard::RequestUser>,
    Json(body): Json<CreateLocationEntity>,
) -> CreateResponse {
    if !permission_check(
        request_user,
        vec![Role::Exec, Role::Team, Role::Tech],
        vec![],
    ) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    }

    match app_state
        .client
        .location()
        .create(body.name, vec![])
        .exec()
        .await
    {
        Ok(_location) => Ok((StatusCode::CREATED, ())),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}
#[axum::debug_handler]
#[utoipa::path(
    get,
    context_path = "/locations",
    path = "",
    responses(
        (status = 200, description = "Get all locations", body = [LocationEntity]),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not found")
    ),
    security(
        ("http" = ["Exec", "Tech", "Team"])
    )
)]
async fn get_all_locations(
    State(app_state): State<AppState>,
    Extension(request_user): Extension<auth_guard::RequestUser>,
) -> GetResponse<Json<Vec<Data>>> {
    if !permission_check(
        request_user,
        vec![Role::Exec, Role::Team, Role::Tech],
        vec![],
    ) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()));
    }
    match app_state.client.location().find_many(vec![]).exec().await {
        Ok(locations) => Ok((StatusCode::OK, Json(locations))),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/locations/{id}",
    responses(
        (status = 200, description = "Get a location by id", body = LocationEntity),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not found")
    ),
    security(
        ("http" = ["Exec", "Tech", "Team"])
    )
)]
async fn get_location_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
    Extension(request_user): Extension<auth_guard::RequestUser>,
) -> GetResponse<Json<Data>> {
    if !permission_check(
        request_user,
        vec![Role::Exec, Role::Team, Role::Tech],
        vec![],
    ) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    }
    match app_state
        .client
        .location()
        .find_unique(prisma::location::UniqueWhereParam::IdEquals(id))
        .exec()
        .await
    {
        Ok(location) => match location {
            Some(location) => Ok((StatusCode::OK, Json(location))),
            None => Err((StatusCode::NOT_FOUND, "Location not found".to_string())),
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
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not found")
    ),
    security(
        ("http" = ["Exec", "Tech", "Team"])
    )
)]
async fn delete_location_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
    Extension(request_user): Extension<auth_guard::RequestUser>,
) -> DeleteResponse {
    if !permission_check(
        request_user,
        vec![Role::Exec, Role::Team, Role::Tech],
        vec![],
    ) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    }
    match app_state
        .client
        .location()
        .delete(prisma::location::UniqueWhereParam::IdEquals(id))
        .exec()
        .await
    {
        Ok(_) => Ok((StatusCode::NO_CONTENT, ())),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}

pub fn location_get_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_all_locations).post(create_location))
        .route(
            "/{id}",
            get(get_location_by_id).delete(delete_location_by_id),
        )
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_guard::require_auth,
        ))
        .with_state(app_state)
}
