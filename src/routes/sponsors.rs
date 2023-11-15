use axum::{
    extract::{Json, State},
    routing::post,
    Router,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    prisma::{
        hackathon::UniqueWhereParam,
        sponsor::{self, Data},
    },
    utils::{get_app_state, AppState},
};

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateSponsorEntity {
    name: String,
    level: String,
    #[validate(url)]
    link: Option<String>,
    dark_logo: String,
    light_logo: String,
    order: i32,
    hackathon_id: Uuid,
}

#[axum::debug_handler]
pub async fn create_sponsor(
    Json(body): Json<CreateSponsorEntity>,
) -> Result<String, (StatusCode, String)> {
    match body.validate() {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
            return Err((StatusCode::BAD_REQUEST, err.to_string()));
        }
    };

    let state = get_app_state().await;

    match state
        .client
        .sponsor()
        .create(
            body.name,
            body.level,
            body.dark_logo,
            body.light_logo,
            body.order,
            UniqueWhereParam::IdEquals(body.hackathon_id.to_string()),
            vec![sponsor::link::set(body.link)],
        )
        .exec()
        .await
    {
        Ok(_sponsor) => Ok("Created sponsor successfully".to_string()),
        Err(_err) => Err((StatusCode::BAD_REQUEST, _err.to_string())),
    }
}

#[axum::debug_handler]
pub async fn get_all_sponsors(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Data>>, String> {
    match app_state.client.sponsor().find_many(vec![]).exec().await {
        Ok(sponsors) => Ok(Json(sponsors)),
        Err(_err) => Err("Error getting all sponsors".to_string()),
    }
}

pub async fn sponsor_get_router() -> Router {
    let state = get_app_state().await;
    Router::new()
        .route("/", post(create_sponsor).get(get_all_sponsors))
        .with_state(state)
}
