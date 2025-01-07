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
/// structure to work with users
/// and explicitly declare
/// them.
use crate::models::KleahUser;

/// Importing the function
/// to send an email to confirm
/// a user's email address.
use crate::email::send_email;

/// Importing the function to retrieve a 
/// user by a token associated with them.
use crate::rw_utils::get_user_by_id;

/// Importing the "StatusResponse"
/// structure to return information
/// on operational success.
use crate::responses::StatusResponse;

/// Importing the structure to submit
/// payloads for updating user information.
use crate::payloads::ChangeEntityPayload;

/// Attempts to update the password for a user.
/// If this operation succeeds,  an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn update_user_password(
    payload: &ChangeEntityPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr>{
    let token: APIToken = match query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let user: KleahUser = match get_user_by_id(&token.user_id, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let verified: bool = match verify(&payload.old_entity, &user.pwd){
        Ok(verified) => verified,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if token.is_active && 
       verified &&
       token.can_change_pwd && 
       token.user_id == user.user_id
    {
        let _update_op: () = match sqlx::query!("UPDATE users SET pwd = $1 WHERE user_id = $2", payload.new_entity, user.user_id)
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
        let e: String = "Token not active or usernames did not match!".to_string();
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to update the mail address for a user.
/// If this operation succeeds, an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn update_user_email(
    payload: &ChangeEntityPayload,
    pool: &Pool<Postgres>,
    smtp_server: &String
) -> Result<StatusResponse, KleahErr>{
    let token: APIToken = match query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let user: KleahUser = match get_user_by_id(&token.user_id, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if token.is_active && 
       token.can_change_pwd && 
       token.user_id == user.user_id
    {
        let hashed_email: String = match hash(&payload.new_entity, DEFAULT_COST){
            Ok(hashed_email) => hashed_email,
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let _update_op: () = match sqlx::query!("UPDATE users SET email = $1 WHERE user_id = $2", hashed_email, user.user_id)
            .execute(pool)
            .await
        {

            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let hashed_email_token = match hash(&format!("{}{}{}", &user.username, &payload.new_entity, get_time()), DEFAULT_COST){
            Ok(hashed) => hashed,
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let _update_token_op: () = match sqlx::query!("UPDATE users SET email_token = $1 WHERE username = $2", hashed_email_token, user.username)
            .execute(pool)
            .await
        {

            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let email_sub: String = format!("Confirm your new email address, {}.", &user.username);
        let from_addr: String = format!("Kleah <noreply@{}>", smtp_server);
        let to_addr: String = format!("{} <{}>", &user.username, &payload.new_entity);
        let message: String = format!("Please copy and paste this link into your browser to confirm your email address: {}/email/verify/{}",smtp_server, hashed_email_token.clone());
        let send_res: bool = match send_email(&from_addr, &to_addr, &email_sub, &message, smtp_server).await {
            Ok(send_res) => send_res,
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        if send_res{
            let status: StatusResponse = StatusResponse{ status: 0};
            Ok(status)
        }
        else {
            let e: String = "Could not send verification email.".to_string();
            Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        }
    }
    else {
        let e: String = "Token not active or usernames did not match!".to_string();
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to update the name for a user.
/// If this operation succeeds, an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn update_user_display_name(
    payload: &ChangeEntityPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr>{
    let token: APIToken = match query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let user: KleahUser = match get_user_by_id(&token.user_id, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if token.is_active && 
       token.can_change_pwd && 
       token.user_id == user.user_id
    {
        let _update_op: () = match query!("UPDATE users SET display_name = $1 WHERE user_id = $2", payload.new_entity, user.user_id)
            .execute(pool)
            .await
        {

            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(StatusResponse{status: 0})
    }
    else {
        let e: String = "Token does not have the correct permissions".to_string();
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// This function attempts to verify the email
/// the user has submitted. If the operation succeeds,
/// a boolean "true" is returned. If the operation fails,
/// an error is returned or a boolean "false" is returned.
pub async fn verify_user_email(
    email_token: &String,
    pool: &Pool<Postgres>
) -> Result<bool, KleahErr> {
    let mut result: bool = false;
    let users: Vec<KleahUser> = match query_as!(KleahUser, "SELECT * FROM users")
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut user_vec: Vec<KleahUser> = Vec::new();
    for user in users {
        if &user.email_token == email_token {
            result = true;
            user_vec.push(user);
        }
        else {}
    }
    let hashed_time: String = match hash(get_time(), DEFAULT_COST){
        Ok(hashed_time) => hashed_time,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user_vec.len() == 1 {}
    else {
        let e: String = "No user with the specified token found.".to_string();
        return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    }
    let user: KleahUser = user_vec[0].clone();
    let _update_op_active: () = match sqlx::query!("UPDATE users SET is_active = $1 WHERE username = $2", true, user.username)
            .execute(pool)
            .await
    {
        Ok(_feedback) => result = true,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let _update_token: () = match sqlx::query!("UPDATE users SET email_token = $1 WHERE username = $2", hashed_time, user.username)
            .execute(pool)
            .await
    {
        Ok(_feedback) => result = true,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(result)
}

/// Attempts to update the avatar for a user.
/// If this operation succeeds, an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn update_user_avatar(
    payload: &ChangeEntityPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr>{
    let token: APIToken = match query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let user: KleahUser = match get_user_by_id(&token.user_id, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if token.is_active && 
       token.can_change_pwd && 
       token.user_id == user.user_id
    {
        let _update_op: () = match query!("UPDATE users SET avatar_url = $1 WHERE user_id = $2", payload.new_entity, user.user_id)
            .execute(pool)
            .await
        {

            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(StatusResponse{status: 0})
    }
    else {
        let e: String = "Token does not have the correct permissions".to_string();
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to update the avatar for a user.
/// If this operation succeeds, an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn update_user_banner(
    payload: &ChangeEntityPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr>{
    let token: APIToken = match query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let user: KleahUser = match get_user_by_id(&token.user_id, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if token.is_active && 
       token.can_change_pwd && 
       token.user_id == user.user_id
    {
        let _update_op: () = match query!("UPDATE users SET banner_url = $1 WHERE user_id = $2", payload.new_entity, user.user_id)
            .execute(pool)
            .await
        {

            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(StatusResponse{status: 0})
    }
    else {
        let e: String = "Token does not have the correct permissions".to_string();
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}