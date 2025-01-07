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
use sqlx::query_as;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::Postgres;

use crate::models::InstanceInfo;
/// Importing this crate's
/// structure to catch and
/// handle errors.
use crate::KleahErr;

/// Importing the "Charm"
/// structure to work with charms
/// and explicitly declare
/// them.
use crate::models::Charm;

/// Importing the "UserLike"
/// structure to work with likes
/// and explicitly declare
/// them.
use crate::models::UserLike;

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

/// Importing the "InviteCode"
/// structure to work with
/// creating invite codes.
use crate::models::InviteCode;

/// Importing the structure
/// that models the promotion
/// of a charm by a user.
use crate::models::Proclamation;

/// Importing the structure
/// that models the action of a
/// user reacting to a charm.
use crate::models::UserReaction;

/// Importing the structure
/// to store files for a user.
use crate::models::KleahUserFile;

/// Importing the structure that
/// models all available reactions
/// on an instance.
use crate::models::InstanceReaction;

/// Attempts to fetch the user with the given ID from the database.
/// If this operation succeeds, an instance of the "KleahUser" structure is
/// returned. If this operation fails, an error is returned. This function
/// is NOT utilized in any API routes.
pub async fn get_user_by_id(
    user_id: &String,
    pool: &Pool<Postgres>
) -> Result<KleahUser, KleahErr> {
    let users: Vec<KleahUser> = match query_as!(KleahUser, "SELECT * FROM users")
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: Vec<KleahUser> = Vec::new();
    for user in users {
        if &user.user_id == user_id {
            result.push(user);
        }
        else {}
    }
    if result.len() == 1{
        Ok(result[0].clone())
    }
    else{
        let e: String = format!("User \"{}\" does not exist.", &user_id);
        Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to fetch the user with the given handle from the database.
/// If this operation succeeds, an instance of the "KleahUser" structure is
/// returned. If this operation fails, an error is returned. This function
/// is NOT utilized in any API routes.
pub async fn get_user_by_handle(
    handle: &String,
    pool: &Pool<Postgres>
) -> Result<KleahUser, KleahErr> {
    let users: Vec<KleahUser> = match query_as!(KleahUser, "SELECT * FROM users")
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: Vec<KleahUser> = Vec::new();
    for user in users {
        if &user.username == handle {
            result.push(user);
        }
        else {}
    }
    if result.len() == 1{
        Ok(result[0].clone())
    }
    else{
        let e: String = format!("User \"{}\" does not exist.", &handle);
        Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// This function attempts to get the
/// user associated with the supplied API
/// token. If this operation succeeds, an
/// instance of the user object whom the token belongs
/// to is supplied. If the operation fails, an error
/// is returned. This function
/// is NOT utilized in any API routes.
pub async fn get_user_from_token(
    api_token: &String, 
    pool: &Pool<Postgres>
) -> Result<KleahUser, KleahErr> {
    let api_tokens: Vec<APIToken> = match query_as!(APIToken, "SELECT * FROM api_tokens")
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut user_id: String = "".to_string();
    for token in api_tokens {
        if &token.token == api_token && token.is_active {
            user_id = token.user_id;
        }
        else {}
    }
    if user_id == "".to_string(){
        let e: String = "No user with the specified API token found.".to_string();
        Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    }
    else {
        let user: KleahUser = match get_user_by_id(&user_id, pool).await {
            Ok(user) => user,
            Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(user)
    }
}

/// This function attempts to get
/// an instance of the "APIToken"
/// structure from the database,
/// given the user's API token.
/// If the operation fails, an error
/// is returned. This function
/// is NOT utilized in any API routes.
pub async fn get_api_token_object(
    token: &String, 
    pool: &Pool<Postgres>
) -> Result<APIToken, KleahErr>{
    let tokens: Vec<APIToken> = match query_as!(APIToken, "SELECT * FROM api_tokens")
        .fetch_all(pool)
        .await
    {
        Ok(tokens) => tokens,
        Err(e) => return Err::<APIToken, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: Vec<APIToken> = Vec::new();
    for stored in tokens {
        if &stored.token == token {
            result.push(stored);
        }
        else {}
    }
    if result.len() == 1 {
        Ok(result[0].clone())
    }
    else {
        let e: String = format!("Token \"{}\" not found.", token);
        Err::<APIToken, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// This function attempts to retrieve a charm
/// given the supplied ID. If this operation fails,
/// an error is returned.
pub async fn get_charm_by_id(
    charm_id: &String, 
    pool: &Pool<Postgres>
) -> Result<Charm, KleahErr>{
    let charms: Vec<Charm> = match query_as!(Charm, "SELECT * FROM charms")
        .fetch_all(pool)
        .await
    {
        Ok(charms) => charms,
        Err(e) => return Err::<Charm, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: Vec<Charm> = Vec::new();
    for stored in charms {
        if &stored.charm_id == charm_id {
            result.push(stored);
        }
        else {}
    }
    if result.len() == 1 {
        Ok(result[0].clone())
    }
    else {
        let e: String = format!("Charm \"{}\" not found.", charm_id);
        Err::<Charm, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to get a specific like given the user's ID 
/// and the charm's ID. If the operation succeeds, an
/// instance of the "UserLike" structure is returned.
/// If the operation fails, an error is returned.
pub async fn get_like_from_charm_and_user(
    charm_id: &String, 
    user_id: &String,
    pool: &Pool<Postgres>
) -> Result<UserLike, KleahErr>{
    let likes: Vec<UserLike> = match query_as!(UserLike, "SELECT * FROM user_likes")
        .fetch_all(pool)
        .await
    {
        Ok(likes) => likes,
        Err(e) => return Err::<UserLike, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut likes_made_by_user: Vec<UserLike> = Vec::new();
    for like in likes {
        if &like.user_id == user_id && &like.charm_id == charm_id {
            likes_made_by_user.push(like);
        }
        else {}
    }
    if likes_made_by_user.len() == 1{
        Ok(likes_made_by_user[0].clone())
    }
    else {
        let e: String = "No likes found.".to_string();
        Err::<UserLike, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to get a specific reaction given the user's ID 
/// and the charm's ID. If the operation succeeds, an
/// instance of the "UserLike" structure is returned.
/// If the operation fails, an error is returned.
pub async fn get_reaction_from_charm_and_user(
    charm_id: &String, 
    user_id: &String,
    pool: &Pool<Postgres>
) -> Result<UserReaction, KleahErr>{
    let reactions: Vec<UserReaction> = match query_as!(UserReaction, "SELECT * FROM user_reactions")
        .fetch_all(pool)
        .await
    {
        Ok(reactions) => reactions,
        Err(e) => return Err::<UserReaction, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut reactions_made_by_user: Vec<UserReaction> = Vec::new();
    for reaction in reactions {
        if &reaction.user_id == user_id && &reaction.charm_id == charm_id {
            reactions_made_by_user.push(reaction);
        }
        else {}
    }
    if reactions_made_by_user.len() == 1{
        Ok(reactions_made_by_user[0].clone())
    }
    else {
        let e: String = "No likes found.".to_string();
        Err::<UserReaction, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to get a specific proclamation given the user's ID 
/// and the charm's ID. If the operation succeeds, an
/// instance of the "UserLike" structure is returned.
/// If the operation fails, an error is returned.
pub async fn get_proclamation_from_charm_and_user(
    charm_id: &String, 
    user_id: &String,
    pool: &Pool<Postgres>
) -> Result<Proclamation, KleahErr>{
    let proclamations: Vec<Proclamation> = match query_as!(Proclamation, "SELECT * FROM user_proclamations")
        .fetch_all(pool)
        .await
    {
        Ok(proclamations) => proclamations,
        Err(e) => return Err::<Proclamation, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut proclamations_made_by_user: Vec<Proclamation> = Vec::new();
    for proclamation in proclamations {
        if &proclamation.user_id == user_id && &proclamation.charm_id == charm_id {
            proclamations_made_by_user.push(proclamation);
        }
        else {}
    }
    if proclamations_made_by_user.len() == 1{
        Ok(proclamations_made_by_user[0].clone())
    }
    else {
        let e: String = "No likes found.".to_string();
        Err::<Proclamation, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

pub async fn get_file_by_id(
    file_id: &String, pool: &Pool<Postgres>
) -> Result<KleahUserFile, KleahErr> {
    let all_files: Vec<KleahUserFile> = match query_as!(KleahUserFile, "SELECT * FROM user_files")
        .fetch_all(pool)
        .await
    {
        Ok(all_files) => all_files,
        Err(e) => return Err::<KleahUserFile, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: Vec<KleahUserFile> = Vec::new();
    for file in all_files{
        if &file.file_id == file_id{
            result.push(file);
        }
        else {}
    }
    if result.len() == 1{
        Ok(result[0].clone())
    }
    else {
        let e: String = format!("A file with the ID\"{}\" does not exist.", &file_id);
        Err::<KleahUserFile, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

pub async fn verify_invite_code(
    invite_code: &String,
    pool: &Pool<Postgres>
) -> Result<bool, KleahErr>{
    let all_codes: Vec<InviteCode> = match query_as!(InviteCode, "SELECT * FROM invite_codes")
        .fetch_all(pool)
        .await
    {
        Ok(all_codes) => all_codes,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: Vec<InviteCode> = Vec::new();
    for code in all_codes{
        if &code.invite_code == invite_code{
            result.push(code);
        }
        else {}
    }
    if result.len() == 1{
            Ok(true)
    }
    else {
        let e: String = format!("A file with the ID\"{}\" does not exist.", &invite_code);
        Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

pub async fn get_reaction_by_name(
    reaction_name: &String,
    pool: &Pool<Postgres>
) -> Result<InstanceReaction, KleahErr>{
    let all_reactions: Vec<InstanceReaction> = match query_as!(InstanceReaction, "SELECT * FROM instance_reactions")
        .fetch_all(pool)
        .await
    {
        Ok(all_reactions) => all_reactions,
        Err(e) => return Err::<InstanceReaction, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: Vec<InstanceReaction> = Vec::new();
    for reaction in all_reactions{
        if &reaction.reaction_name == reaction_name{
            result.push(reaction);
        }
        else {}
    }
    if result.len() == 1{
            Ok(result[0].clone())
    }
    else {
        let e: String = format!("A reaction with the name \"{}\" does not exist.", &reaction_name);
        Err::<InstanceReaction, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to get the instance's admin as an instance
/// of the "KleahUser" structure. If this operation fails, 
/// an error is returned.
pub async fn get_instance_admin(pool: &Pool<Postgres>) -> Result<KleahUser, KleahErr>{
    let users: Vec<KleahUser> = match query_as!(KleahUser, "SELECT * FROM users")
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: Vec<KleahUser> = Vec::new();
    for user in users {
        if user.is_admin{
            result.push(user);
        }
    }
    if result.len() == 1{
        Ok(result[0].clone())
    }
    else {
        let e: String = "No admin user exists.".to_string();
        Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to get the instance's ID (hostname) as a string.
/// If this operation fails, an error is returned.
pub async fn get_instance_hostname(pool: &Pool<Postgres>) -> Result<String, KleahErr> {
    let user: KleahUser = match get_instance_admin(pool).await {
        Ok(user) => user,
        Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let info: InstanceInfo = match query_as!(InstanceInfo, "SELECT * FROM instance_information WHERE admin_user_id = $1", user.user_id)
        .fetch_one(pool)
        .await 
    {
        Ok(info) => info,
        Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(info.instance_id)
}