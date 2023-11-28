/*
Things we need in auth:
Verify if user we received from header is in database
 */

use serde::{Deserialize, Serialize};

use crate::prisma::Role;

pub fn auth_user_from_header(id_token: String, roles: Vec<Role>) -> bool {
    //check if user exists with firebase
    //check if user exists in database
    //check if user has role
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

/* localId	string	The uid of the current user.
email	string	The email of the account.
emailVerified	boolean	Whether or not the account's email has been verified.
displayName	string	The display name for the account.
providerUserInfo	List of JSON objects	List of all linked provider objects which contain "providerId" and "federatedId".
photoUrl	string	The photo Url for the account.
passwordHash	string	Hash version of password.
passwordUpdatedAt	double	The timestamp, in milliseconds, that the account password was last changed.
validSince	string	The timestamp, in seconds, which marks a boundary, before which Firebase ID token are considered revoked.
disabled	boolean	Whether the account is disabled or not.
lastLoginAt	string	The timestamp, in milliseconds, that the account last logged in at.
createdAt	string	The timestamp, in milliseconds, that the account was created at.
customAuth	boolean	Whether the account is authenticated by the developer. */

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct FirebaseUserResponse {
    local_id: String,
    email: String,
    email_verified: bool,
    display_name: String,
    provider_user_info: Vec<String>,
    photo_url: String,
    password_hash: String,
    password_updated_at: String,
    valid_since: String,
    disabled: bool,
    last_login_at: String,
    created_at: String,
    custom_auth: bool,
}
