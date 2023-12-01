use axum::{
    extract::{Path, State},
    response::Response,
    Json,
};
use hyper::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    base_types::{AppState, StandardResponse},
    prisma::{organizer::Data, Role},
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
) -> Result<Response<String>, StatusCode> {
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
        Ok(_) => Ok(Response::new("Created Organizer successfully".to_string())),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn get_all_organizers(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Data>>, StatusCode> {
    match app_state.client.organizer().find_many(vec![]).exec().await {
        Ok(organizers) => Ok(Json(organizers)),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn get_organizer_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Data>, StatusCode> {
    match app_state
        .client
        .organizer()
        .find_unique(crate::prisma::organizer::UniqueWhereParam::IdEquals(id))
        .exec()
        .await
    {
        Ok(organizer) => match organizer {
            Some(organizer) => Ok(Json(organizer)),
            None => Err(StatusCode::NOT_FOUND),
        },
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}


