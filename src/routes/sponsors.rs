use axum::{
    extract::State,
    routing::get,
    Json, Router,
};

use crate::{
    prisma::{hackathon, sponsor},
    utils::{self, AppState},
};

#[axum::debug_handler]
async fn sponsor_create(
    State(app_state): State<AppState>,
    Json(data): Json<sponsor::Data>,
) -> Json<sponsor::Data> {
    Json(
        app_state
            .client
            .sponsor()
            .create(
                data.name,
                data.level,
                data.dark_logo,
                data.light_logo,
                data.order,
                hackathon::UniqueWhereParam::IdEquals(data.hackathon_id),
                vec![],
            )
            .exec()
            .await
            .unwrap(),
    )
}

pub async fn sponsor_get_router() -> Router {
    let state = utils::get_app_state().await;
    Router::new()
        .route("/", get(sponsor_create))
        .with_state(state)
}
