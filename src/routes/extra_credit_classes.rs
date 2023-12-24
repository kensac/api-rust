use axum::{
    extract::{Path, State},
    response::Response,
    routing::get,
    Json, Router,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    base_types::AppState,
    prisma::{self, extra_credit_class::Data},
};

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateExtraCreditClassEntity {
    name: String,
    hackathon_id: Uuid,
}
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/extra_credit_class",
    responses(
        (status = 200, description = "Create a new extra credit class", body = String),
        (status = 400, description = "Bad request")
    ),
    request_body = CreateExtraCreditClassEntity
)]
async fn create_extra_credit_class(
    State(app_state): State<AppState>,
    Json(body): Json<CreateExtraCreditClassEntity>,
) -> Result<Response<String>, StatusCode> {
    match app_state
        .client
        .extra_credit_class()
        .create(
            body.name,
            prisma::hackathon::UniqueWhereParam::IdEquals(body.hackathon_id.to_string()),
            vec![],
        )
        .exec()
        .await
    {
        Ok(_hackathon) => Ok(Response::new(
            "Created extra credit class successfully".to_owned(),
        )),
        Err(_err) => Err(StatusCode::BAD_REQUEST),
    }
}
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/extra_credit/classes",
    responses(
        (status = 200, description = "Get all extra credit classes"),
        (status = 400, description = "Bad request")
    )
)]
async fn get_all_extra_credit_classes(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Data>>, StatusCode> {
    match app_state
        .client
        .extra_credit_class()
        .find_many(vec![])
        .exec()
        .await
    {
        Ok(extra_credit_classes) => Ok(Json(extra_credit_classes)),
        Err(_err) => Err(StatusCode::BAD_REQUEST),
    }
}
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/extra_credit/classes/{id}",
    responses(
        (status = 200, description = "Get a extra credit class by id"),
        (status = 400, description = "Bad request")
    )
)]
async fn get_extra_credit_class_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Data>, (StatusCode, String)> {
    match app_state
        .client
        .extra_credit_class()
        .find_unique(prisma::extra_credit_class::UniqueWhereParam::IdEquals(id))
        .exec()
        .await
    {
        Ok(extra_credit_class) => match extra_credit_class {
            Some(extra_credit_class) => Ok(Json(extra_credit_class)),
            None => Err((StatusCode::NOT_FOUND, "".to_owned())),
        },
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}

#[axum::debug_handler]
#[utoipa::path(
    delete,
    path = "/extra_credit/classes/{id}",
    responses(
        (status = 200, description = "Delete a extra credit class by id"),
        (status = 400, description = "Bad request")
    )
)]
async fn delete_extra_credit_class_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    match app_state
        .client
        .extra_credit_class()
        .delete(prisma::extra_credit_class::UniqueWhereParam::IdEquals(id))
        .exec()
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}

pub fn extra_credit_class_get_router(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(get_all_extra_credit_classes).post(create_extra_credit_class),
        )
        .route(
            "/:id",
            get(get_extra_credit_class_by_id).delete(delete_extra_credit_class_by_id),
        )
        .with_state(app_state)
}
