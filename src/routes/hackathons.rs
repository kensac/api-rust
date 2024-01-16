use std::vec;

use axum::{
    extract::{Extension, Path, Query, State},
    middleware,
    routing::{get, patch, post},
    Json, Router,
};
use hyper::StatusCode;
use utoipa::IntoParams;

use crate::{
    auth_guard::{self, permission_check, RequestUser},
    base_types::AppState,
    base_types::{CreateResponse, DeleteResponse, GetResponse, UpdateResponse},
    entities::hackathons::CreateHackathonEntity,
    prisma::{
        hackathon::{self, Data, UniqueWhereParam},
        location, EventType, Role,
    },
};

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
    if !permission_check(request_user, Role::Exec, vec![]) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()));
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
                    "Hackathon CheckIn".to_owned(),
                    EventType::CheckIn,
                    "CheckIn for Hackathon".to_owned(),
                    location::UniqueWhereParam::IdEquals("0".to_owned()),
                    body.start_time,
                    body.end_time,
                    hackathon::UniqueWhereParam::IdEquals(hackathon.id),
                    vec![],
                )
                .exec()
                .await;

            match event {
                Ok(_) => Ok((StatusCode::CREATED, ())),
                Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
            }
        }
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
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
    if !permission_check(request_user, Role::Team, vec![]) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()));
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
            Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
        }
    } else {
        match app_state.client.hackathon().find_many(vec![]).exec().await {
            Ok(hackathons) => Ok((StatusCode::OK, Json(hackathons))),
            Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
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
    if !permission_check(request_user, Role::Team, vec![]) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()));
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
            None => Err((StatusCode::NOT_FOUND, "No hackathon found".to_owned())),
        },
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
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
    if !permission_check(request_user, Role::Exec, vec![]) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()));
    }
    match app_state
        .client
        .hackathon()
        .delete(UniqueWhereParam::IdEquals(id))
        .exec()
        .await
    {
        Ok(_) => Ok((StatusCode::NO_CONTENT, ())),
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
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
    if !permission_check(request_user, Role::Exec, vec![]) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()));
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
        Err(err) => return Err((StatusCode::BAD_REQUEST, err.to_string())),
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
        Err(err) => Err((StatusCode::BAD_REQUEST, err.to_string())),
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
            None => Err((StatusCode::NOT_FOUND, "No hackathon found".to_owned())),
        },
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub fn hackathon_get_router(app_state: AppState) -> Router {
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
