use crate::AppState;

use crate::database::users_db::{
    create_user, get_first_user, get_user_by_first_name, get_user_by_id,
};
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};

#[derive(Debug, serde::Deserialize)]

pub struct UserRequest {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[axum::debug_handler]
pub async fn create_user_route(
    State(state): State<AppState>,
    Json(data): Json<UserRequest>,
) -> Result<(), ()> {
    println!("{:?}", data);
    let _user = create_user(state.client, data.first_name, data.last_name).await;
    Ok(())
}

#[axum::debug_handler]
pub async fn get_first_user_route(State(state): State<AppState>) -> Json<UserResponse> {
    let _user = get_first_user(state.client).await;
    Json(UserResponse {
        id: _user.id,
        first_name: _user.first_name,
        last_name: _user.last_name,
    })
}

#[axum::debug_handler]
pub async fn get_user_by_id_route(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Json<UserResponse> {
    let id = user_id;
    let _user = get_user_by_id(state.client, id).await;
    Json(UserResponse {
        id: _user.id,
        first_name: _user.first_name,
        last_name: _user.last_name,
    })
}

pub async fn get_user_by_first_name_route(
    State(state): State<AppState>,
    Path(first_name): Path<String>,
) -> Json<UserResponse> {
    let _user = get_user_by_first_name(state.client, first_name).await;
    Json(UserResponse {
        id: _user.id,
        first_name: _user.first_name,
        last_name: _user.last_name,
    })
}

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users/", post(create_user_route).get(get_first_user_route))
        .route("/users/:user_id", get(get_user_by_id_route))
        .route("/users/:first_name", get(get_user_by_first_name_route))
}