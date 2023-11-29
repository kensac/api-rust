/*
Things we need in auth:
Verify if user we received from header is in database
 */

use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    prisma::{organizer, Role},
    utils::AppState,
};

pub async fn auth_user_from_header(
    app_state: AppState,
    id_token: String,
    roles: Vec<Role>,
) -> bool {
    if roles.len() == 0 {
        return true;
    }

    //check if user exists with firebase
    let user_data = reqwest::Client::new()
        .post(std::env::var("FIREBASE_USER_DATA_ENDPOINT").unwrap())
        .query(&[("key", std::env::var("FIREBASE_API_KEY").unwrap())])
        .json(&serde_json::json!({
            "idToken": id_token
        }))
        .send()
        .await
        .unwrap();

    if user_data.status() != StatusCode::OK {
        return false;
    }

    let res: FirebaseUserResult = serde_json::from_str(&user_data.text().await.unwrap()).unwrap();

    let user_uid = res.users[0].local_id.clone();

    match app_state
        .client
        .organizer()
        .find_unique(organizer::UniqueWhereParam::GcpIdEquals(user_uid))
        .exec()
        .await
        .unwrap()
    {
        Some(organizer) => {
            for role in roles {
                if organizer.privilege != role {
                    return false;
                }
            }
        }
        None => return false,
    }

    true
}

pub fn auth_user_from_header_with_restrictions(
    id_token: String,
    roles: Vec<Role>,
    restrictions: fn(FirebaseUserResponse) -> bool,
) -> bool {
    //check if user exists with firebase
    //check if user exists in database
    //check if user has role
    //check if user has restrictions
    let user = FirebaseUserResponse::default();
    if !restrictions(user) {
        return false;
    }
    true
}

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
