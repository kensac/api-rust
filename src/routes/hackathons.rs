use std::vec;

use axum::{
    extract::{Path, Query, State},
    response::Response,
    routing::{get, patch, post},
    Json, Router,
};
use chrono::FixedOffset;
use hyper::StatusCode;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    prisma::hackathon::{self, Data, UniqueWhereParam},
    utils::{get_app_state, AppState},
};

#[derive(serde::Deserialize, ToSchema)]
pub struct CreateHackathonEntity {
    name: String,
    start_time: chrono::DateTime<FixedOffset>,
    end_time: chrono::DateTime<FixedOffset>,
}

#[derive(serde::Deserialize, IntoParams)]
struct Params {
    #[serde(default)]
    active: Option<bool>,
}

#[axum::debug_handler]
#[utoipa::path(post, path = "/hackathon", responses((status = 200, description = "Create a new hackathon", body = String), (status=400, description = "Bad request")), request_body = CreateHackathonEntity)]
async fn create_hackathon(
    State(app_state): State<AppState>,
    Json(body): Json<CreateHackathonEntity>,
) -> Result<Response<String>, StatusCode> {
    //add event that also serves as check-in for hackathon
    match app_state
        .client
        .hackathon()
        .create(Uuid::new_v4().to_string(), body.name, body.start_time, body.end_time, false, vec![])
        .exec()
        .await
    {
        Ok(_) => Ok(Response::new("Created hackathon successfully".to_string())),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

#[axum::debug_handler]
#[utoipa::path(get, path = "/hackathon", responses((status = 200, description = "Returns all hackathons"), (status=404, description = "No hackathon found")) , params(Params), request_body = CreateHackathonEntity)]
async fn get_hackathon(
    State(app_state): State<AppState>,
    Query(params): Query<Params>,
) -> Result<Json<Vec<Data>>, StatusCode> {
    if params.active.is_some() {
        match app_state
            .client
            .hackathon()
            .find_many(vec![hackathon::active::equals(params.active.unwrap())])
            .exec()
            .await
        {
            Ok(hackathons) => Ok(Json(hackathons)),
            Err(_) => Err(StatusCode::NOT_FOUND),
        }
    } else {
        match app_state.client.hackathon().find_many(vec![]).exec().await {
            Ok(hackathons) => Ok(Json(hackathons)),
            Err(_) => Err(StatusCode::NOT_FOUND),
        }
    }
}

#[axum::debug_handler]
#[utoipa::path(get, path = "/hackathon/{id}", responses((status = 200, description = "Returns hackathon with id"), (status=404, description = "No hackathon found")), params(("id" = String, Path, description = "id of hackathon to get")))]
async fn get_hackathon_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<Data>>, StatusCode> {
    match app_state
        .client
        .hackathon()
        .find_unique(UniqueWhereParam::IdEquals(id))
        .exec()
        .await
    {
        Ok(hackathons) => match hackathons {
            Some(hackathon) => Ok(Json(vec![hackathon])),
            None => Err(StatusCode::NOT_FOUND),
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[axum::debug_handler]
#[utoipa::path(delete, path = "/hackathon/{id}", responses((status = 204, description = "Delete hackathon with id")))]
async fn delete_hackathon_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match app_state
        .client
        .hackathon()
        .delete(UniqueWhereParam::IdEquals(id))
        .exec()
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[axum::debug_handler]
#[utoipa::path(post, path = "/hackathon/{id}/active", responses((status = 200, description = "Set hackathon with id to active")))]
async fn set_active_hackathon(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match app_state
        .client
        .hackathon()
        .update(
            UniqueWhereParam::IdEquals(id),
            vec![hackathon::active::set(true)],
        )
        .exec()
        .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn hackathon_get_router() -> Router {
    let state = get_app_state().await;
    Router::new()
        .route("/", post(create_hackathon).get(get_hackathon))
        .route(
            "/:id",
            get(get_hackathon_by_id).delete(delete_hackathon_by_id),
        )
        .route("/:id/active", patch(set_active_hackathon))
        .with_state(state)
}
