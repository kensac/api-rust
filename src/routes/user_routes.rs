use crate::AppState;

use crate::database::users_db::{create_user, get_first_user};
use axum::Json;
use axum::extract::State;

#[derive(Debug)]
#[derive(serde::Deserialize)]

pub struct UserRequest {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[axum::debug_handler]
pub async fn create_user_route(State(state): State<AppState> , Json(data): Json<UserRequest>) -> Result<(), ()>{
    println!("{:?}", data);
    let _user = create_user(state.client, data.first_name, data.last_name).await;
    Ok(())
}

#[axum::debug_handler]
pub async fn get_first_user_route(State(state): State<AppState>) -> Json<UserResponse>{
    let _user = get_first_user(state.client).await;
    Json(UserResponse {
        id: _user.id,
        first_name: _user.first_name,
        last_name: _user.last_name,
    })
}