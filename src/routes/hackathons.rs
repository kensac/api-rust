use std::vec;

use axum::{
    extract::{Extension, Path, Query, State},
    middleware,
    routing::{get, patch, post},
    Json, Router,
};
use chrono::FixedOffset;
use hyper::StatusCode;
use utoipa::{IntoParams, ToSchema};

use crate::{
    auth_guard::{self, permission_check, RequestUser},
    base_types::AppState,
    base_types::{CreateResponse, DeleteResponse, GetResponse, UpdateResponse},
    prisma::{
        hackathon::{self, Data, UniqueWhereParam},
        location, EventType, Role,
    },
};

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
/* #[serde(remote = "Data")] */
pub struct HackathonEntity {
    _id: String,
    _name: String,
    _start_time: chrono::DateTime<FixedOffset>,
    _end_time: chrono::DateTime<FixedOffset>,
    _active: bool,
    /*     event: Option<Vec<prisma::event::Data>>,
    extra_credit_class: Option<Vec<prisma::extra_credit_class::Data>>,
    project: Option<Vec<prisma::project::Data>>,
    sponsor: Option<Vec<prisma::sponsor::Data>>,
    registration: Option<Vec<prisma::registration::Data>>,
    scan: Option<Vec<prisma::scan::Data>>,
    score: Option<Vec<prisma::score::Data>>, */
}

#[derive(serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
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
#[utoipa::path(
    post,
    context_path = "/hackathons",
    path = "",
    responses(
        (status = 201, description = "Created a new hackathon"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    ),
    request_body = CreateHackathonEntity,
    security(
        ("api_key" = ["Exec", "Tech"])
    )
)]
async fn create_hackathon(
    State(app_state): State<AppState>,
    Extension(request_user): Extension<RequestUser>,
    Json(body): Json<CreateHackathonEntity>,
) -> CreateResponse {
    if !permission_check(request_user, vec![Role::Exec, Role::Tech], vec![]) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    }
    match app_state
        .client
        .hackathon()
        .create(body.name, body.start_time, body.end_time, false, vec![])
        .exec()
        .await
    {
        Ok(hackathon) => {
            let event = app_state
                .client
                .event()
                .create(
                    "Hackathon CheckIn".to_string(),
                    EventType::CheckIn,
                    "CheckIn for Hackathon".to_string(),
                    location::UniqueWhereParam::IdEquals("0".to_string()),
                    body.start_time,
                    body.end_time,
                    hackathon::UniqueWhereParam::IdEquals(hackathon.id),
                    vec![],
                )
                .exec()
                .await;

            match event {
                Ok(_) => Ok((StatusCode::CREATED, ())),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            }
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[axum::debug_handler]
#[utoipa::path(
    get,
    context_path = "/hackathons",
    path = "",
    responses(
        (status = 200, description = "Returns all hackathons", body = [HackathonEntity]),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
    ),
    params(Params),
    security(
                ("api_key" = ["Exec", "Tech"])
    )
)]
async fn get_all_hackathon(
    State(app_state): State<AppState>,
    Query(params): Query<Params>,
    Extension(request_user): Extension<RequestUser>,
) -> GetResponse<Json<Vec<Data>>> {
    if !permission_check(
        request_user,
        vec![Role::Exec, Role::Team, Role::Tech],
        vec![],
    ) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    }

    if params.active.is_some() {
        match app_state
            .client
            .hackathon()
            .find_many(vec![hackathon::active::equals(params.active.unwrap())])
            .exec()
            .await
        {
            Ok(hackathons) => Ok((StatusCode::OK, Json(hackathons))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    } else {
        match app_state.client.hackathon().find_many(vec![]).exec().await {
            Ok(hackathons) => Ok((StatusCode::OK, Json(hackathons))),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
        }
    }
}

#[axum::debug_handler]
#[utoipa::path(
    get,
    context_path = "/hackathons",
    path = "/{id}",
    responses(
        (status = 200, description = "Returns hackathon with id", body = HackathonEntity),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "No hackathon found")
    ),
    params(("id" = String, Path, description = "id of hackathon to get")),
    security(
                ("api_key" = ["Exec", "Tech"])
    )
)]
async fn get_hackathon_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
    Extension(request_user): Extension<RequestUser>,
) -> GetResponse<Json<Data>> {
    if !permission_check(
        request_user,
        vec![Role::Exec, Role::Team, Role::Tech],
        vec![],
    ) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    }
    match app_state
        .client
        .hackathon()
        .find_unique(UniqueWhereParam::IdEquals(id))
        .exec()
        .await
    {
        Ok(hackathons) => match hackathons {
            Some(hackathon) => Ok((StatusCode::OK, Json(hackathon))),
            None => Err((StatusCode::NOT_FOUND, "No hackathon found".to_string())),
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[axum::debug_handler]
#[utoipa::path(
    delete,
    context_path = "/hackathons",
    path = "/{id}",
    responses((status = 204, description = "Delete hackathon with id"),
    (status = 400, description = "Bad request"),
    (status = 401, description = "Unauthorized"),
    ),
    params(("id" = String, Path, description = "id of hackathon to delete")),
    security(
                ("api_key" = ["Exec", "Tech"])
    )
)]
async fn delete_hackathon_by_id(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
    Extension(request_user): Extension<RequestUser>,
) -> DeleteResponse {
    if !permission_check(request_user, vec![Role::Exec, Role::Tech], vec![]) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    }
    match app_state
        .client
        .hackathon()
        .delete(UniqueWhereParam::IdEquals(id))
        .exec()
        .await
    {
        Ok(_) => Ok((StatusCode::NO_CONTENT, ())),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    context_path = "/hackathons",
    path = "/{id}/active",
    responses((status = 200, description = "Set hackathon with id to active"),
    (status = 400, description = "Bad request"),
    (status = 401, description = "Unauthorized"),
    ),
    params(("id" = String, Path, description = "id of hackathon to set active")),
    security(
                ("api_key" = ["Exec", "Tech"]
    )
))]
async fn set_active_hackathon(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
    Extension(request_user): Extension<RequestUser>,
) -> UpdateResponse {
    if !permission_check(request_user, vec![Role::Exec, Role::Tech], vec![]) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()));
    }
    //set all hackathons to inactive
    match app_state
        .client
        .hackathon()
        .update_many(vec![], vec![hackathon::active::set(false)])
        .exec()
        .await
    {
        Ok(_) => (),
        Err(e) => return Err((StatusCode::BAD_REQUEST, e.to_string())),
    }

    //set hackathon with id to active
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
        Ok(_) => Ok((StatusCode::OK, ())),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[axum::debug_handler]
#[utoipa::path(
    get,
    context_path = "/hackathons",
    path = "/active/static",
    responses((status = 200, description = "Returns active hackathon", body = HackathonEntity),
    (status = 400, description = "Bad request"),
    (status = 404, description = "No hackathon found")
    ),
    security(
                ()
    )
)]
async fn get_active_hackathon(State(app_state): State<AppState>) -> GetResponse<Json<Data>> {
    match app_state
        .client
        .hackathon()
        .find_first(vec![hackathon::active::equals(true)])
        .exec()
        .await
    {
        Ok(hackathons) => match hackathons {
            Some(hackathon) => Ok((StatusCode::OK, Json(hackathon))),
            None => Err((StatusCode::NOT_FOUND, "No hackathon found".to_string())),
        },
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn hackathon_get_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", post(create_hackathon).get(get_all_hackathon))
        .route(
            "/:id",
            get(get_hackathon_by_id).delete(delete_hackathon_by_id),
        )
        .route("/:id/active", patch(set_active_hackathon))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_guard::require_auth,
        ))
        .route("/active/static", get(get_active_hackathon))
        .with_state(app_state)
}
