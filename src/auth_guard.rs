use std::future::Future;

use axum::{extract::State, http::HeaderValue, middleware::Next, response::Response, Router};
use hyper::{HeaderMap, Request, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{
    base_types::{AppState, APP_STATE},
    prisma::{user, Role},
};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirebaseUserResponse {
    local_id: String,
    email: String,
    email_verified: bool,
    valid_since: String,
    disabled: bool,
    last_login_at: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FirebaseUserResult {
    kind: String,
    users: Vec<FirebaseUserResponse>,
}

fn extract_auth_header(headers: &HeaderMap) -> Result<String, StatusCode> {
    let header = headers
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let header_str = header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;
    let parts: Vec<&str> = header_str.split_whitespace().collect();
    parts
        .get(1)
        .cloned()
        .map(|s| s.to_string())
        .ok_or(StatusCode::UNAUTHORIZED)
}

async fn fetch_firebase_user(
    auth_header: &str,
    app_state: &AppState,
) -> Result<FirebaseUserResult, StatusCode> {
    let user_data = app_state
        .reqwest_client
        .post(
            std::env::var("FIREBASE_USER_DATA_ENDPOINT")
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        )
        .query(&[(
            "key",
            std::env::var("FIREBASE_API_KEY").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        )])
        .json(&serde_json::json!({ "idToken": auth_header }))
        .send()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if user_data.status() != reqwest::StatusCode::OK {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        serde_json::from_str(
            &user_data
                .text()
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn require_auth(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = extract_auth_header(&headers)?;
    let firebase_user = fetch_firebase_user(&auth_header, &app_state).await?;

    let user_uid = firebase_user
        .users
        .first()
        .map(|user| user.local_id.clone())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user = app_state
        .client
        .user()
        .find_unique(user::UniqueWhereParam::GcpIdEquals(user_uid))
        .exec()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}

type Predicate = (Role, Box<dyn Fn(user::Data) -> bool>);
pub type RequestUser = user::Data;

pub fn permission_check(
    user: RequestUser,
    unrestricted_role: Role,
    additional_check: Vec<Predicate>,
) -> bool {
    {
        if user.privilege >= unrestricted_role {
            return true;
        }
        for (role, check) in additional_check {
            if user.privilege == role && check(user.clone()) {
                return true;
            }
        }
        false
    }
}

pub async fn permission_check_socket(
    headers: HeaderMap<HeaderValue>,
    unrestricted_role: Role,
) -> bool {
    let auth_header = match extract_auth_header(&headers) {
        Ok(header) => header,
        Err(_) => return false,
    };

    let app_state = match APP_STATE.get() {
        Some(state) => state,
        None => return false,
    };

    let firebase_user = match fetch_firebase_user(&auth_header, app_state).await {
        Ok(user) => user,
        Err(_) => return false,
    };

    let user_uid = match firebase_user.users.first() {
        Some(user) => user.local_id.clone(),
        None => return false,
    };

    match app_state
        .client
        .user()
        .find_unique(user::UniqueWhereParam::GcpIdEquals(user_uid))
        .exec()
        .await
    {
        Ok(Some(user)) => user.privilege >= unrestricted_role,
        _ => false,
    }
}

pub fn permission_check_async<T>(
    _user: RequestUser,
    _organizer_roles: Vec<Role>,
    _user_additional_check: fn(user::Data) -> T,
) -> bool
where
    T: Future<Output = bool>,
{
    todo!()
}

// Doesn't work that's why it's private. Will try to fix later as that will reduce code duplication
fn _async_auth_router_layer() -> Router {
    todo!()
}
