use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use hyper::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    base_types::{AppState, CreateResponse, DeleteResponse, GetResponse},
    prisma::{
        organizer::{Data, UniqueWhereParam},
        Role,
    },
};

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct CreateOrganizerEntity {
    first_name: String,
    last_name: String,
    email: String,
    privilege: Role,
    gcp_id: String,
}

async fn create_organizer(
    State(app_state): State<AppState>,
    Json(body): Json<CreateOrganizerEntity>,
) -> CreateResponse {
    match app_state
        .client
        .organizer()
        .create(
            body.first_name,
            body.last_name,
            body.email,
            body.privilege,
            body.gcp_id,
            vec![],
        )
        .exec()
        .await
    {
        Ok(_organizer) => Ok((StatusCode::CREATED, ())),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}

async fn get_all_organizers(State(app_state): State<AppState>) -> GetResponse<Json<Vec<Data>>> {
    match app_state.client.organizer().find_many(vec![]).exec().await {
        Ok(organizers) => Ok((StatusCode::OK, Json(organizers))),
        Err(err) => Err((StatusCode::NOT_FOUND, err.to_string())),
    }
}

async fn get_organizer_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> GetResponse<Json<Data>> {
    match app_state
        .client
        .organizer()
        .find_unique(crate::prisma::organizer::UniqueWhereParam::IdEquals(id))
        .exec()
        .await
    {
        Ok(organizer) => match organizer {
            Some(organizer) => Ok((StatusCode::OK, Json(organizer))),
            None => Err((StatusCode::NOT_FOUND, "No organizer found".to_owned())),
        },
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

async fn delete_organizer_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> DeleteResponse {
    match app_state
        .client
        .organizer()
        .delete(UniqueWhereParam::IdEquals(id))
        .exec()
        .await
    {
        Ok(_) => Ok((StatusCode::NO_CONTENT, ())),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}

pub fn routes(app_state: AppState) -> Router {
    axum::Router::new()
        .route("/", post(create_organizer))
        .route("/", get(get_all_organizers))
        .route("/:id", get(get_organizer_by_id))
        .route("/:id", delete(delete_organizer_by_id))
        .with_state(app_state)
}
