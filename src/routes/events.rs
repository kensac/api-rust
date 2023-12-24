use axum::{
    debug_handler,
    extract::{Path, State},
    middleware,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use chrono::FixedOffset;
use hyper::StatusCode;

use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    auth_guard::{self, permission_check, RequestUser},
    base_types::{AppState, CreateResponse, DeleteResponse, GetResponse},
    prisma::{self, event, hackathon, location, EventType},
};

impl<'__s> utoipa::ToSchema<'__s> for EventType {
    fn schema() -> (
        &'__s str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            "EventType",
            utoipa::openapi::ObjectBuilder::new()
                .schema_type(utoipa::openapi::SchemaType::String)
                .enum_values::<[&str; 4usize], &str>(Some([
                    "Activity", "Food", "Workshop", "CheckIn",
                ]))
                .into(),
        )
    }

    fn aliases() -> Vec<(&'__s str, utoipa::openapi::schema::Schema)> {
        vec![]
    }
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EventEntity {
    _id: Uuid,
    _name: String,
    r#_type: EventType,
    _description: String,
    _location_id: Uuid,
    _icon: Option<String>,
    _start_time: chrono::DateTime<FixedOffset>,
    _end_time: chrono::DateTime<FixedOffset>,
    _ws_presenter_names: Option<String>,
    _ws_relevant_skills: Option<String>,
    _ws_skill_level: Option<String>,
    _ws_urls: Option<String>,
    _hackathon_id: Uuid,
}

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

#[debug_handler]
#[utoipa::path(
    post,
    path = "/event",
    responses(
        (status = 201, description = "Created a new event"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    ),
    request_body = CreateEventEntity,
    security(
        ("http" = ["Exec", "Tech", "Team"])
    )
)]
pub async fn create_event(
    State(app_state): State<AppState>,
    Extension(request_user): Extension<RequestUser>,
    Json(body): Json<CreateEventEntity>,
) -> CreateResponse {
    if !permission_check(
        request_user,
        vec![prisma::Role::Exec, prisma::Role::Team, prisma::Role::Tech],
        vec![],
    ) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()));
    }

    match app_state
        .client
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
                event::ws_urls::set(body.ws_urls),
            ],
        )
        .exec()
        .await
    {
        Ok(_event) => Ok((StatusCode::CREATED, ())),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}

#[debug_handler]
#[utoipa::path(
    get,
    context_path = "/events",
    path = "",
    responses(
        (status = 200, description = "Get all events", body = [EventEntity]),
        (status = 400, description = "Bad request"),
    ),
    security(
        ()
    )
)]
pub async fn get_all_events(
    State(app_state): State<AppState>,
) -> GetResponse<Json<Vec<event::Data>>> {
    match app_state
        .client
        .event()
        .find_many(vec![])
        .with(event::scan::fetch(vec![]))
        .exec()
        .await
    {
        Ok(events) => Ok((StatusCode::OK, Json(events))),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}

#[debug_handler]
#[utoipa::path(
    get,
    context_path = "/events",
    path = "/:event_id",
    responses(
        (status = 200, description = "Get event by id", body = EventEntity),
        (status = 400, description = "Bad request"),
        (status = 404, description = "No event found"),
    ),
    security(
        ()
    )
)]
pub async fn get_event_by_id(
    State(app_state): State<AppState>,
    Path(event_id): Path<Uuid>,
) -> GetResponse<Json<event::Data>> {
    match app_state
        .client
        .event()
        .find_unique(event::UniqueWhereParam::IdEquals(event_id.to_string()))
        .exec()
        .await
    {
        Ok(event) => match event {
            Some(event) => Ok((StatusCode::OK, Json(event))),
            None => Err((StatusCode::NOT_FOUND, "No event found".to_owned())),
        },
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}

#[debug_handler]
#[utoipa::path(
    delete,
    context_path = "/events",
    path = "/:event_id",
    responses(
        (status = 204, description = "Deleted event by id"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("http" = ["Exec", "Tech", "Team"])
    )
)]
pub async fn delete_event_by_id(
    State(app_state): State<AppState>,
    Path(event_id): Path<Uuid>,
) -> DeleteResponse {
    match app_state
        .client
        .event()
        .delete(event::UniqueWhereParam::IdEquals(event_id.to_string()))
        .exec()
        .await
    {
        Ok(_) => Ok((StatusCode::NO_CONTENT, ())),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CheckInUserToEventEntity {
    hackathon_id: Uuid,
    organizer_id: Uuid,
}

#[debug_handler]
#[utoipa::path(
    post,
    context_path = "/events",
    path = "/:event_id/check-in/user/:user_id",
    responses(
        (status = 200, description = "Checked in user to event"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
    ),
    request_body = CheckInUserToEventEntity,
    security(
        ()
    )
)]
pub async fn check_in_user_to_event(
    State(app_state): State<AppState>,
    Path((event_id, registration_id)): Path<(Uuid, Uuid)>,
    Extension(request_user): Extension<RequestUser>,
    Json(body): Json<CheckInUserToEventEntity>,
) -> CreateResponse {
    if !permission_check(
        request_user,
        vec![prisma::Role::Exec, prisma::Role::Team, prisma::Role::Tech],
        vec![],
    ) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()));
    }
    match app_state
        .client
        .scan()
        .create(
            prisma::registration::UniqueWhereParam::IdEquals(registration_id.to_string()),
            prisma::organizer::UniqueWhereParam::IdEquals(body.organizer_id.to_string()),
            body.hackathon_id.to_string(),
            event::UniqueWhereParam::IdEquals(event_id.to_string()),
            vec![],
        )
        .exec()
        .await
    {
        Ok(_) => Ok((StatusCode::OK, ())),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
    }
}

pub fn events_get_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", post(create_event))
        .route("/:event_id", delete(delete_event_by_id))
        .route(
            "/:event_id/check-in/user/:user_id",
            post(check_in_user_to_event),
        )
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_guard::require_auth,
        ))
        .route("/", get(get_all_events))
        .route("/:event_id", get(get_event_by_id))
        .with_state(app_state)
}
