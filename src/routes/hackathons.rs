use std::vec;

use axum::{extract::{State, Path}, response::Response, routing::{post, get}, Router, Json};
use chrono::FixedOffset;

use crate::{utils::{get_app_state, AppState}, prisma::hackathon::{Data, UniqueWhereParam}};

#[derive(serde::Deserialize)]
struct CreateHackatonEntity {
    name: String,
    start_time: chrono::DateTime<FixedOffset>,
    end_time: chrono::DateTime<FixedOffset>,
}

#[axum::debug_handler]
async fn create_hackaton(State(app_state): State<AppState>, Json(body): Json<CreateHackatonEntity>) -> Response<String> {
    match app_state.client.hackathon().create(body.name, body.start_time, body.end_time,true,vec![]).exec().await {
        Ok(_) => {
            Response::new("Created hackathon succesfully".to_string())},
        Err(_) => {
            Response::new("Failed to create hackathon".to_string())
        },
    }
}

#[axum::debug_handler]
async fn get_hackaton(State(app_state): State<AppState>) ->  Result<Json<Vec<Data>>,() >{
    match app_state.client.hackathon().find_many(vec![]).exec().await {
        Ok(hackathons) => {
            Ok(Json(hackathons))
        },
        Err(_) => {
            Err(())
        },
    }
}

#[axum::debug_handler]
async fn get_hackaton_by_id(State(app_state): State<AppState>, Path(id): Path<i32>) ->  Result<Json<Vec<Data>>,() >{
    match app_state.client.hackathon().find_unique(UniqueWhereParam::IdEquals(id)).exec().await {
        Ok(hackathons) => {
            match hackathons {
                Some(hackathon) => {
                    Ok(Json(vec![hackathon]))
                },
                None => {
                    Err(())
                }
            }
        },
        Err(_) => {
            Err(())
        },
    }
}

pub async fn hackathon_get_router() -> Router {
    let state = get_app_state().await;
    Router::new()
        .route("/", post(create_hackaton).get(get_hackaton))
        .route("/:id", get(get_hackaton_by_id))
        .with_state(state)
}
