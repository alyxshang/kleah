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

/// Importing the "Charm"
/// structure to work with charms
/// and explicitly declare
/// them.
use crate::models::Charm;

use crate::keys::KeyPair;

use crate::keys::generate_key_pair;

/// Importing the "KleahUser"
/// structure to work with users
/// and explicitly declare
/// them.
use crate::models::KleahUser;

/// Importing the function
/// to send an email to confirm
/// a user's email address.
use crate::email::send_email;

/// Importing the function to check
/// whether an admin user already exists.
use crate::admin::admin_exists;

/// Importing the structure that
/// returns a user profile.
use crate::responses::UserProfile;

/// Importing the function to retrieve
/// a user by their ID.
use crate::rw_utils::get_user_by_id;

/// Importing the structure that allows
/// one to submit a payload for retrieving
/// profile information about a user.
use crate::payloads::ProfilePayload;

/// Importing the structure that
/// models a follow relationship
/// between two users.
use crate::models::KleahUserFollows;

/// Importing the "StatusResponse"
/// structure to return information
/// on operational success.
use crate::responses::StatusResponse;

/// Importing the structure for the payload
/// to delete a user.
use crate::payloads::DeleteUserPayload;

/// Importing the structure for the payload
/// to create a user.
use crate::payloads::CreateUserPayload;

/// Importing the function to retrieve a 
/// user by a token associated with them.
use crate::rw_utils::get_user_from_token;

use super::rw_utils::get_instance_hostname;

/// Attempts to create a new user with the given payload.
/// If this operation succeeds, an instance of the "KleahUser" structure is
/// returned. If this operation fails, an error is returned.
pub async fn write_user(
    payload: &CreateUserPayload,
    pool: &Pool<Postgres>,
    smtp_server: &String,
    host_server: &String
) -> Result<KleahUser, KleahErr> {
    let hashed_pwd = match hash(payload.pwd.clone(), DEFAULT_COST){
        Ok(hashed) => hashed,
        Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let key_pair: KeyPair = match generate_key_pair(){
        Ok(key_pair) => key_pair,
        Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let user_id_string: String = format!("{}:{}", &payload.username, get_time());
    let user_id = match hash(&user_id_string, DEFAULT_COST){
        Ok(user_id) => user_id,
        Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let hashed_email_token: String = match hash(&format!("{}{}{}", &payload.username, &payload.email, get_time()), DEFAULT_COST){
        Ok(hashed) => hashed,
        Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let hashed_email = match hash(&payload.email, DEFAULT_COST){
        Ok(hashed) => hashed,
        Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let admin_exists: bool = match admin_exists(pool).await {
        Ok(admin_exists) => admin_exists,
        Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let user_already_exists: bool = match user_exists(&payload.username, pool).await {
        Ok(user_exists) => user_exists,
        Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let user_role: String;
    if payload.user_role == "normal".to_string(){
        user_role = "Mage".to_string();
    }
    else if payload.user_role == "admin".to_string() && admin_exists == false {
        user_role = "Tianquan".to_string();
    }
    else {
        let e: &str = "Invalid role supplied.";
        return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()));
    }
    if user_already_exists{
        let e: String = format!("The user \"{}\" already exists.", &payload.username);
        return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()));
    }
    else {}
    let host: String = match get_instance_hostname(pool).await {
        Ok(host) => host,
        Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let new_user: KleahUser = KleahUser{
        user_id: user_id.clone(),
        user_role: user_role,
        username: payload.username.clone(),
        display_name: payload.display_name.clone(),
        avatar_url: payload.avatar_url.clone(),
        banner_url: payload.banner_url.clone(),
        user_description: payload.user_description.clone(),
        email: hashed_email,
        pwd: hashed_pwd,
        host: host,
        priv_key: key_pair.private_key,
        pub_key: key_pair.public_key,
        is_private: payload.is_private,
        email_token: hashed_email_token.clone(),
        is_active: false,
        rules_accepted: payload.rules_accepted,
        is_admin: payload.is_admin
    };

    let _insert_op = match query!(
        "INSERT INTO users (user_id, user_role, username, display_name, avatar_url, banner_url, user_description, email, pwd, host, priv_key, pub_key, is_private, email_token, is_active, rules_accepted, is_admin) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)",
        new_user.user_id,
        new_user.user_role,
        new_user.username,
        new_user.display_name,
        new_user.avatar_url,
        new_user.banner_url,
        new_user.user_description,
        new_user.email,
        new_user.pwd,
        new_user.host,
        new_user.priv_key,
        new_user.pub_key,
        new_user.is_private,
        new_user.email_token,
        new_user.is_active,
        new_user.rules_accepted,
        new_user.is_admin
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let email_sub: String = format!("Confirm your email address, {}.", &payload.username);
    let from_addr: String = format!("Jade <noreply@{}>", host_server);
    let to_addr: String = format!("{} <{}>", &payload.username, &payload.email);
    let message: String = format!("Please copy and paste this link into your browser to confirm your email address: {}/email/verify/{}",host_server, hashed_email_token.clone());
    let send_res: bool = match send_email(&from_addr, &to_addr, &email_sub, &message, smtp_server).await {
        Ok(send_res) => send_res,
        Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if send_res{
        let res: KleahUser = match get_user_by_id(&user_id.clone(), pool).await {
            Ok(res) => res,
            Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(res)
    }
    else {
        let e: String = "Could not send verification email.".to_string();
        Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to delete a user given one of their API tokens.
/// If this operation succeeds, an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn wipe_user(
    payload: &DeleteUserPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr> {
    let user: KleahUser = match get_user_from_token(&payload.api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user.username == payload.username {
        let _wipe_op: () = match query!("DELETE FROM users WHERE user_id = $1", user.user_id)
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
        let e: String = "Only the owner of the token can delete their account.".to_string();
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to collect all profile information on a user.
/// If this operation is successful, an instance of the 
/// "UserProfile" structure is returned. If this operation
/// is not successful, an error is returned.
pub async fn assemble_profile(
    payload: &ProfilePayload,
    pool: &Pool<Postgres>
) -> Result<UserProfile, KleahErr> {
    let user: KleahUser = match get_user_from_token(&payload.api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<UserProfile, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user.is_active{
        let user_charms: Vec<Charm> = match query_as!(Charm, "SELECT * FROM charms WHERE user_id = $1", user.user_id)
        .fetch_all(pool)
        .await
        {
            Ok(users) => users,
            Err(e) => return Err::<UserProfile, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let charm_count: usize = user_charms.len();
        let following_users: Vec<KleahUserFollows> = match query_as!(KleahUserFollows, "SELECT * FROM user_follows WHERE follower = $1", user.user_id)
            .fetch_all(pool)
            .await
        {
            Ok(users) => users,
            Err(e) => return Err::<UserProfile, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let following_users_count: usize = following_users.len();
        let followers_users: Vec<KleahUserFollows> = match query_as!(KleahUserFollows, "SELECT * FROM user_follows WHERE followee = $1", user.user_id)
            .fetch_all(pool)
            .await
        {
            Ok(users) => users,
            Err(e) => return Err::<UserProfile, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let followers_users_count: usize = followers_users.len();
        let result: UserProfile = UserProfile{
            user_role: user.user_role,
            username: user.username,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            banner_url: user.banner_url,
            user_description: user.user_description,
            follower_count: followers_users_count,
            following_count: following_users_count,
            charm_count: charm_count
        };
        Ok(result)
    }
    else {
        let e: String = format!("User \"{}\" needs to be verified to have a public profile.", &user.username);
        Err::<UserProfile, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to check whether a user exists.
/// If they do, a boolean "true" is returned. If they do
/// not a boolean "false" is returned. If any other failures 
/// occur, an error is returned.
pub async fn user_exists(username: &String,pool: &Pool<Postgres>) -> Result<bool, KleahErr> {
    let mut result: bool = false;
    let all_users: Vec<KleahUser> = match query_as!(KleahUser, "SELECT * FROM users")
        .fetch_all(pool)
        .await
    {
        Ok(all_users) => all_users,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    for user in all_users {
        if &user.username == username{ result = true; }
        else {}
    }
    Ok(result)
}