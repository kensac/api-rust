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

pub async fn require_auth(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = match headers.get("Authorization") {
        Some(header) => match header.to_str() {
            Ok(header_str) => {
                let parts = header_str.split(' ').collect::<Vec<&str>>();
                if parts.len() > 1 {
                    parts[1]
                } else {
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }
            Err(_) => return Err(StatusCode::UNAUTHORIZED),
        },
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let user_data = app_state
        .reqwest_client
        .post(std::env::var("FIREBASE_USER_DATA_ENDPOINT").unwrap())
        .query(&[("key", std::env::var("FIREBASE_API_KEY").unwrap())])
        .json(&serde_json::json!({
            "idToken": auth_header
        }))
        .send()
        .await
        .unwrap();

    if user_data.status() != reqwest::StatusCode::OK {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let firebase_user: FirebaseUserResult =
        serde_json::from_str(&user_data.text().await.unwrap()).unwrap();

    // this might not be the best way to check the user's id because it checks only the first user in the list
    let user_uid = firebase_user
        .users
        .first()
        .map(|user| user.local_id.clone())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    match app_state
        .client
        .user()
        .find_unique(user::UniqueWhereParam::GcpIdEquals(user_uid.clone()))
        .exec()
        .await
    {
        Ok(user) => match user {
            Some(user) => {
                request.extensions_mut().insert(user);
            }
            None => return Err(StatusCode::UNAUTHORIZED),
        },
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    Ok(next.run(request).await)
}

type Predicate = (Role, Box<dyn Fn(user::Data) -> bool>);
pub type RequestUser = user::Data;

pub fn permission_check(
    user: RequestUser,
    unrestricted_roles: Vec<Role>,
    additional_check: Vec<Predicate>,
) -> bool {
    {
        for role in unrestricted_roles {
            if user.privilege == role {
                return true;
            }
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
    unrestricted_roles: Vec<Role>,
) -> bool {
    let auth_header = match headers.get("Authorization") {
        Some(header) => match header.to_str() {
            Ok(header_str) => {
                let parts = header_str.split(' ').collect::<Vec<&str>>();
                if parts.len() > 1 {
                    parts[1]
                } else {
                    return false;
                }
            }
            Err(_) => return false,
        },
        None => return false,
    };

    let app_state = match APP_STATE.get() {
        Some(state) => state,
        None => return false,
    };

    let user_data = app_state
        .reqwest_client
        .post(std::env::var("FIREBASE_USER_DATA_ENDPOINT").unwrap())
        .query(&[("key", std::env::var("FIREBASE_API_KEY").unwrap())])
        .json(&serde_json::json!({
            "idToken": auth_header
        }))
        .send()
        .await
        .unwrap();

    if user_data.status() != reqwest::StatusCode::OK {
        return false;
    }

    let firebase_user: FirebaseUserResult =
        serde_json::from_str(&user_data.text().await.unwrap()).unwrap();

    let user_uid = firebase_user
        .users
        .first()
        .map(|user| user.local_id.clone())
        .ok_or(false);

    match app_state
        .client
        .user()
        .find_unique(user::UniqueWhereParam::GcpIdEquals(
            user_uid.unwrap().clone(),
        ))
        .exec()
        .await
    {
        Ok(user) => match user {
            Some(user) => {
                for role in unrestricted_roles {
                    if user.privilege == role {
                        return true;
                    }
                }
                false
            }
            None => false,
        },
        Err(_) => false,
    }
}

pub async fn permission_check_async<T>(
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
async fn _auth_router_layer() -> Router {
    todo!()
}
