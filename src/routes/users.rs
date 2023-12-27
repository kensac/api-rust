// Import necessary modules
use crate::{
    auth_guard::{self, permission_check, RequestUser},
    base_types::{AppState, CreateResponse, DeleteResponse, GetResponse},
    prisma::{user, Role},
};
use axum::{
    debug_handler,
    extract::{Extension, Path, State},
    middleware,
    routing::{get, post},
    Json, Router,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

// User entity for serialization/deserialization
#[derive(Serialize, Deserialize)]
pub struct UserEntity {
    id: String,
    first_name: String,
    last_name: String,
    email: String,
    gcp_id: String,
}

// Define a struct for creating a new user
#[derive(Deserialize, Serialize)]
pub struct CreateUserEntity {
    first_name: String,
    last_name: String,
    email: String,
    gcp_id: String,
}

// Route handler to create a new user
#[debug_handler]
async fn create_user(
    State(app_state): State<AppState>,
    Json(data): Json<CreateUserEntity>,
) -> CreateResponse {
    // Database operation
    match app_state
        .client
        .user()
        .create(
            data.first_name,
            data.last_name,
            data.email,
            data.gcp_id,
            vec![],
        )
        .exec()
        .await
    {
        Ok(_) => Ok((StatusCode::CREATED, ())),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
#[debug_handler]
async fn get_all_users(
    State(app_state): State<AppState>,
    Extension(request_user): Extension<RequestUser>,
) -> GetResponse<Json<Vec<user::Data>>> {
    if !(permission_check(
        request_user,
        vec![Role::Exec, Role::Team, Role::Tech],
        vec![],
    )) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()));
    }
    // Replace with actual Prisma client logic
    match app_state.client.user().find_many(vec![]).exec().await {
        Ok(users) => Ok((StatusCode::OK, Json(users))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

#[debug_handler]
async fn get_user_by_id(
    State(app_state): State<AppState>,
    Path(user_id): Path<String>,
    Extension(request_user): Extension<RequestUser>,
) -> GetResponse<Json<user::Data>> {
    let permission_id = user_id.clone();

    if !(permission_check(
        request_user,
        vec![Role::Team, Role::Tech, Role::Exec],
        vec![(
            Role::None,
            Box::new(move |user: user::Data| -> bool { permission_id == user.id }),
        )],
    )) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()));
    }
    match app_state
        .client
        .user()
        .find_unique(user::UniqueWhereParam::IdEquals(user_id))
        .exec()
        .await
    {
        Ok(user) => match user {
            Some(user) => Ok((StatusCode::OK, Json(user))),
            None => Err((StatusCode::NOT_FOUND, "User not found".to_owned())),
        },
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

// Route handler to delete a user by ID
async fn delete_user_by_id(
    State(app_state): State<AppState>,
    Path(user_id): Path<String>,
    Extension(request_user): Extension<RequestUser>,
) -> DeleteResponse {
    if !(permission_check(
        request_user,
        vec![Role::Exec, Role::Team, Role::Tech],
        vec![],
    )) {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_owned()));
    };
    match app_state
        .client
        .user()
        .delete(user::UniqueWhereParam::IdEquals(user_id))
        .exec()
        .await
    {
        Ok(_) => Ok((StatusCode::OK, ())),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

// Function to setup user routes
pub fn user_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/:id", get(get_user_by_id).delete(delete_user_by_id))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_guard::require_auth,
        ))
        .route("/", get(get_all_users))
        .with_state(app_state)
}
