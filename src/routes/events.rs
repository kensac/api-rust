use axum::{ extract::{ Path, State }, Json, Router, routing::{post, get}, debug_handler };
use chrono::FixedOffset;
use hyper::StatusCode;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{ prisma::{ self, event, hackathon, location, EventType }, utils::{AppState, get_app_state} };

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventEntity {
    name: String,
    r#type: EventType,
    description: String,
    location_id: Uuid,
    icon: Option<String>,
    start_time: chrono::DateTime<FixedOffset>,
    end_time: chrono::DateTime<FixedOffset>,
    ws_presenter_names: Option<String>,
    ws_relevant_skills: Option<String>,
    ws_skill_level: Option<String>,
    ws_urls: Option<String>,
    hackathon_id: Uuid,
}

pub async fn create_event(
    State(app_state): State<AppState>,
    Json(body): Json<CreateEventEntity>
) -> Result<String, (StatusCode, String)> {
    match
        app_state.client
            .event()
            .create(
                body.name,
                body.r#type,
                body.description,
                location::UniqueWhereParam::IdEquals(body.location_id.to_string()),
                body.start_time,
                body.end_time,
                hackathon::UniqueWhereParam::IdEquals(body.hackathon_id.to_string()),
                vec![
                    event::icon::set(body.icon),
                    event::ws_presenter_names::set(body.ws_presenter_names),
                    event::ws_relevant_skills::set(body.ws_relevant_skills),
                    event::ws_skill_level::set(body.ws_skill_level),
                    event::ws_urls::set(body.ws_urls)
                ]
            )
            .exec().await
    {
        Ok(_event) => Ok("Created event successfully".to_string()),
        Err(_err) => Err((StatusCode::BAD_REQUEST, _err.to_string())),
    }
}

#[debug_handler]
pub async fn get_events(State(app_state): State<AppState>) -> Result<
    Json<Vec<event::Data>>,
    (StatusCode, String)
> {
    match
        app_state.client
            .event()
            .find_many(vec![])
            .with(event::scan::fetch(vec![]))
            .exec().await
    {
        Ok(events) => Ok(Json(events)),
        Err(_err) => Err((StatusCode::BAD_REQUEST, _err.to_string())),
    }
}

pub async fn get_event_by_id(
    State(app_state): State<AppState>,
    Path(event_id): Path<Uuid>
) -> Result<Json<event::Data>, (StatusCode, String)> {
    match
        app_state.client
            .event()
            .find_unique(event::UniqueWhereParam::IdEquals(event_id.to_string()))
            .exec().await
    {
        Ok(event) =>
            match event {
                Some(event) => Ok(Json(event)),
                None => Err((StatusCode::NOT_FOUND, "No event found".to_string())),
            }
        Err(_err) => Err((StatusCode::BAD_REQUEST, _err.to_string())),
    }
}

pub async fn delete_event_by_id(
    State(app_state): State<AppState>,
    Path(event_id): Path<Uuid>
) -> Result<StatusCode, (StatusCode, String)> {
    match
        app_state.client
            .event()
            .delete(event::UniqueWhereParam::IdEquals(event_id.to_string()))
            .exec().await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_err) => Err((StatusCode::BAD_REQUEST, _err.to_string())),
    }
}

#[derive(serde::Deserialize, ToSchema)]
pub struct CheckInUserToEventEntity {
    hackathon_id: Uuid,
    organizer_id: Uuid,
}

pub async fn check_in_user_to_event(
    State(app_state): State<AppState>,
    Path((event_id, user_id)): Path<(Uuid,Uuid)>,
    Json(body): Json<CheckInUserToEventEntity>
) -> Result<String, (StatusCode, String)> {
    match
        app_state.client
            .scan()
            .create(
                prisma::user::UniqueWhereParam::IdEquals(user_id.to_string()),
                prisma::organizer::UniqueWhereParam::IdEquals(body.organizer_id.to_string()),
                body.hackathon_id.to_string(),
                event::UniqueWhereParam::IdEquals(event_id.to_string()),
                vec![]
            )
            .exec().await
    {
        Ok(_) => Ok("Checked in user to event successfully".to_string()),
        Err(_err) => Err((StatusCode::BAD_REQUEST, _err.to_string())),
    }
}

pub async fn events_get_router() -> Router {
    let state = get_app_state().await;

    Router::new()
        .route("/", post(create_event).get(get_events))
        .route("/:event_id", get(get_event_by_id).delete(delete_event_by_id))
        .route("/:event_id/check-in/user/:user_id", post(check_in_user_to_event))
        .with_state(state)
}