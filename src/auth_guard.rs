use std::future::Future;

use axum::{
    extract::State,
    middleware::{self, Next},
    response::Response,
    Router,
};
use hyper::{HeaderMap, Request, StatusCode};
use serde::{Deserialize, Serialize};
use tokio::try_join;

use crate::{
    base_types::AppState,
    prisma::{organizer, user, Role},
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
        Some(header) => header.to_str().unwrap().split(' ').collect::<Vec<&str>>()[1],
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
    let user_uid = firebase_user.users[0].local_id.clone();

    let state1 = app_state.clone();
    let state2 = app_state.clone();

    // Start both queries in parallel
    let organizer_future = state1
        .client
        .organizer()
        .find_unique(organizer::UniqueWhereParam::GcpIdEquals(user_uid.clone()))
        .exec();

    let user_future = state2
        .client
        .user()
        .find_unique(user::UniqueWhereParam::GcpIdEquals(user_uid.clone()))
        .exec();

    // Wait for both futures to complete
    match try_join!(organizer_future, user_future) {
        Ok((Some(organizer), Some(user))) => {
            request
                .extensions_mut()
                .insert(RequestUser::Organizer(organizer));
            request.extensions_mut().insert(RequestUser::User(user));
        }
        Ok((Some(organizer), None)) => {
            request
                .extensions_mut()
                .insert(RequestUser::Organizer(organizer));
        }
        Ok((None, Some(user))) => {
            request.extensions_mut().insert(RequestUser::User(user));
        }
        Ok((None, None)) => {
            // Handle case where both are None, if necessary
        }
        Err(_e) => {
            // Handle error, if necessary
        }
    }

    // Check if both organizer and user are not present
    if request.extensions().get::<RequestUser>().is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}

#[derive(Debug, Clone)]
pub enum RequestUser {
    Organizer(organizer::Data),
    User(user::Data),
}

/*
    To use permission_check on any route, you need to extract the user with the following code:
    Extension(user): Extension<RequestUser>
    Then you can use the permission_check function like this:
    permission_check(user, vec![Role::Admin], |user| {user.id == params.id})
*/
type Predicate = (Role, Box<dyn Fn(RequestUser) -> bool>);

pub fn permission_check(
    user: RequestUser,
    unrestricted_roles: Vec<Role>,
    additional_check: Vec<Predicate>,
) -> bool {
    match user {
        RequestUser::Organizer(organizer) => {
            for role in unrestricted_roles {
                if organizer.privilege == role {
                    return true;
                }
            }
            for (role, check) in additional_check {
                if organizer.privilege == role && check(RequestUser::Organizer(organizer.clone())) {
                    return true;
                }
            }
            false
        }
        RequestUser::User(user) => {
            for role in unrestricted_roles {
                if user.privilege == role {
                    return true;
                }
            }
            for (role, check) in additional_check {
                if user.privilege == role && check(RequestUser::User(user.clone())) {
                    return true;
                }
            }
            false
        }
    }
}

/* Async version of the code is available in case you need to do async checks.
Not sure if it works. I think I added the right traits to make it work but we'll find
out when we try to use it.
 */
pub async fn permission_check_async<T>(
    user: RequestUser,
    organizer_roles: Vec<Role>,
    user_additional_check: fn(user::Data) -> T,
) -> bool
where
    T: Future<Output = bool>,
{
    match user {
        RequestUser::Organizer(organizer) => {
            for role in organizer_roles {
                if organizer.privilege != role {
                    return false;
                }
            }
            true
        }
        RequestUser::User(user) => user_additional_check(user).await,
    }
}

// Doesn't work that's why it's private. Will try to fix later as that will reduce code duplication
async fn _auth_router_layer() -> Router {
    let state = AppState::new().await;

    Router::new().route_layer(middleware::from_fn_with_state(state.clone(), require_auth))
}
