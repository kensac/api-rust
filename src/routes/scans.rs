use axum::{ extract::{ State, Path }, Json, Router, routing::get };
use hyper::StatusCode;
use serde::Deserialize;
use uuid::Uuid;
use utoipa::ToSchema;

use crate::{ utils::{ AppState, get_app_state }, prisma::{ scan, event } };

pub async fn get_all_scans(State(app_state): State<AppState>) -> Result<
    Json<Vec<scan::Data>>,
    StatusCode
> {
    match
        app_state.client
            .scan()
            .find_many(vec![])
            .exec().await
    {
        Ok(scans) => Ok(Json(scans)),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScanIdEntity {
    event_id: Uuid,
    user_id: Uuid,
}

pub async fn get_scan_by_id(
    State(app_state): State<AppState>,
    Path(path): Path<ScanIdEntity>
) -> Result<Json<Vec<scan::Data>>, StatusCode> {
    match
        app_state.client
            .scan()
            .find_unique(
                scan::UniqueWhereParam::EventIdUserIdEquals(
                    path.event_id.to_string(),
                    path.user_id.to_string()
                )
            )
            .exec().await
    {
        Ok(scan) =>
            match scan {
                Some(scan) => Ok(Json(vec![scan])),
                None => Err(StatusCode::NOT_FOUND),
            }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_scans_by_organizer_id(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<Json<Vec<scan::Data>>, StatusCode> {
    match
        app_state.client
            .scan()
            .find_many(vec![scan::organizer_id::equals(id.to_string())])
            .exec().await
    {
        Ok(scans) => Ok(Json(scans)),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn get_scans_by_user_id(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<Json<Vec<scan::Data>>, StatusCode> {
    match
        app_state.client
            .scan()
            .find_many(vec![scan::user_id::equals(id.to_string())])
            .exec().await
    {
        Ok(scans) => Ok(Json(scans)),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn get_all_events_with_scans(State(app_state): State<AppState>) -> Result<
    Json<Vec<event::Data>>,
    StatusCode
> {
    match
        app_state.client
            .event()
            .find_many(vec![])
            .with(event::scan::fetch(vec![]))
            .exec().await
    {
        Ok(events) => Ok(Json(events)),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn get_event_with_scans_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<Json<Vec<event::Data>>, StatusCode> {
    match
        app_state.client
            .event()
            .find_unique(event::UniqueWhereParam::IdEquals(id.to_string()))
            .with(event::scan::fetch(vec![]))
            .exec().await
    {
        Ok(event) =>
            match event {
                Some(event) => Ok(Json(vec![event])),
                None => Err(StatusCode::NOT_FOUND),
            }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn scans_get_router() -> Router {
    let state = get_app_state().await;
    Router::new()
        .route("/", get(get_all_scans))
        .route("/:event_id/:user_id", get(get_scan_by_id))
        .route("/analytics/organizer/:id", get(get_scans_by_organizer_id))
        .route("/analytics/user/:id", get(get_scans_by_user_id))
        .route("/analytics/events", get(get_all_events_with_scans))
        .route("/analytics/events/:id", get(get_event_with_scans_by_id))
        .with_state(state)
}
