use std::vec;

use axum::{
    extract::{Path, Query, State},
    response::Response,
    routing::{get, post},
    Json, Router,
};
use chrono::FixedOffset;

use crate::{
    prisma::hackathon::{self, Data, UniqueWhereParam},
    utils::{get_app_state, AppState},
};

#[derive(serde::Deserialize)]
struct CreateHackathonEntity {
    name: String,
    start_time: chrono::DateTime<FixedOffset>,
    end_time: chrono::DateTime<FixedOffset>,
}

#[derive(serde::Deserialize)]
struct Params {
    #[serde(default)]
    active: Option<bool>,
}

#[axum::debug_handler]
async fn create_hackathon(
    State(app_state): State<AppState>,
    Json(body): Json<CreateHackathonEntity>,
) -> Response<String> {
    match app_state
        .client
        .hackathon()
        .create(body.name, body.start_time, body.end_time, true, vec![])
        .exec()
        .await
    {
        Ok(_) => Response::new("Created hackathon successfully".to_string()),
        Err(_) => Response::new("Failed to create hackathon".to_string()),
    }
}

#[axum::debug_handler]
async fn get_hackathon(
    State(app_state): State<AppState>,
    Query(params): Query<Params>,
) -> Result<Json<Vec<Data>>, ()> {
    if params.active.is_some() {
        match app_state
            .client
            .hackathon()
            .find_many(vec![hackathon::active::equals(params.active.unwrap())])
            .exec()
            .await
        {
            Ok(hackathons) => Ok(Json(hackathons)),
            Err(_) => Err(()),
        }
    } else {
        match app_state.client.hackathon().find_many(vec![]).exec().await {
            Ok(hackathons) => Ok(Json(hackathons)),
            Err(_) => Err(()),
        }
    }
}

#[axum::debug_handler]
async fn get_hackathon_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Vec<Data>>, ()> {
    match app_state
        .client
        .hackathon()
        .find_unique(UniqueWhereParam::IdEquals(id))
        .exec()
        .await
    {
        Ok(hackathons) => match hackathons {
            Some(hackathon) => Ok(Json(vec![hackathon])),
            None => Err(()),
        },
        Err(_) => Err(()),
    }
}

pub async fn hackathon_get_router() -> Router {
    let state = get_app_state().await;
    Router::new()
        .route("/", post(create_hackathon).get(get_hackathon))
        .route("/:id", get(get_hackathon_by_id))
        .with_state(state)
}
