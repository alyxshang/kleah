/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// "Pool" structure
/// from the "sqlx" crate
/// to make a pool for
/// database connections.
use sqlx::Pool;

/// Importing the macro
/// from the "sqlx" crate
/// to execute SQL queries.
use sqlx::query;

/// Importing the 
/// "hash" function
/// to hash strings.
use bcrypt::hash;

/// Importing the macro
/// from the "sqlx" crate
/// to execute SQL queries.
use sqlx::query_as;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::Postgres;

/// Importing the 
/// "verify" function
/// to verify strings.
use bcrypt::verify;

/// Importing this crate's
/// structure to catch and
/// handle errors.
use crate::KleahErr;

/// Importing the "DEFAULT_COST"
/// entity to hash strings.
use bcrypt::DEFAULT_COST;

/// Importing the "get_time"
/// function to get the current
/// time stamp.
use crate::time::get_time;

/// Importing the "APIToken"
/// structure to work with API
/// tokens and explicitly declare
/// them.
use crate::models::APIToken;

/// Importing the "KleahUser"
/// structure to work with Kleah
/// users and explicitly declare
/// them.
use crate::models::KleahUser;

/// Importing a function to generate
/// a random string.
use crate::utils::generate_chars;

/// Importing the "StatusResponse"
/// structure to return information
/// on operational success.
use crate::responses::StatusResponse;

/// Importing the function to retrieve a 
/// user by their user ID.
use crate::rw_utils::get_user_by_id;

/// Importing the structure to obtain
/// all active tokens a user created.
use crate::payloads::UserTokensPayload;

/// Importing the function to retrieve
/// a user's profile information given
/// their handle.
use super::rw_utils::get_user_by_handle;

/// Importing the structure to delete
/// a token a user created.
use crate::payloads::DeleteUserTokenPayload;

/// Importing the structure to create
/// a new token.
use crate::payloads::CreateUserTokenPayload;

/// Attempts to create a new API token for a user with
/// the given payload. If this operation succeeds, 
/// an instance of  the "JadeMood" structure. If this 
/// operation fails, an  error is returned.
pub async fn create_new_token(
    payload: &CreateUserTokenPayload,
    pool: &Pool<Postgres>
) -> Result<APIToken, KleahErr> {
    let user: KleahUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<APIToken, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let is_valid: bool = match verify(&user.pwd,&payload.password){
        Ok(is_valid) => is_valid,
        Err(e) => return Err::<APIToken, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if is_valid {
        let hashed: String = match hash(format!("{}:{}:{}", get_time(), &payload.username, generate_chars(16)), DEFAULT_COST){
            Ok(hashed) => hashed,
            Err(e) => return Err::<APIToken, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let new_token: APIToken = APIToken{
            user_id: user.user_id.clone(),
            token: hashed,
            created_at: get_time(),
            is_active: true,
            can_change_username: payload.can_change_username,
            can_change_pwd: payload.can_change_pwd,
            can_post_charms: payload.can_post_charms,
            can_delete_user: payload.can_delete_user,
            can_change_email: payload.can_change_email.clone(),
        };
        let _insert_op = match query!(
            "INSERT INTO api_tokens (user_id, token, created_at, is_active, can_change_username, can_change_pwd, can_post_charms, can_delete_user, can_change_email) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            new_token.user_id,
            new_token.token,
            new_token.created_at,
            new_token.is_active,
            new_token.can_change_username,
            new_token.can_change_pwd,
            new_token.can_post_charms,
            new_token.can_delete_user,
            new_token.can_change_email
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<APIToken, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(new_token)

    }
    else {
        let e: String = format!("Passwords did not match for user \"{}\"!", &payload.username);
        Err::<APIToken, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to delete an API token of a user.
/// If this operation succeeds,  an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn wipe_token(
    payload: &DeleteUserTokenPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr> {
    let user: KleahUser = match get_user_by_handle(&payload.user_id, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let verified: bool = match verify(payload.password.clone(), &user.pwd){
        Ok(verified) => verified,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if verified{
        let _wipe_op: () = match query!("DELETE FROM api_tokens WHERE token = $1", payload.api_token)
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let status: StatusResponse = StatusResponse{ status: 0 };
        Ok(status)
    }
    else {
        let e: String = format!("Passwords did not match for user \"{}\"!", &payload.user_id);
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to retrieve all active API tokens for a user.
/// If this operation is successful, a vector of the
/// instances of the "APIToken" structure is returned.
/// If this operation fails, an error is returned.
pub async fn get_user_tokens(
    payload: &UserTokensPayload,
    pool: &Pool<Postgres>
) -> Result<Vec<APIToken>, KleahErr>{
    let user: KleahUser = match get_user_by_id(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Vec<APIToken>, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let verified: bool = match verify(payload.password.clone(), &user.pwd){
        Ok(verified) => verified,
        Err(e) => return Err::<Vec<APIToken>, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if verified {
        let tokens: Vec<APIToken>  = match query_as!(APIToken, "SELECT * FROM api_tokens")
            .fetch_all(pool)
            .await
        {
            Ok(tokens) => tokens,
            Err(e) => return Err::<Vec<APIToken>, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let mut result: Vec<APIToken> = Vec::new();
        for token in tokens {
            if token.user_id == user.user_id {
                if token.is_active {}
                else {
                    result.push(token);
                }
            }
            else {}
        }
        Ok(result)
    }
    else {
        let e: String = format!("Passwords do not match for user \"{}\"!", &user.username);
        Err::<Vec<APIToken>, KleahErr>(KleahErr::new(&e.to_string()))
    }
}