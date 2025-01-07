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

/// Importing the "Charm"
/// structure to work with charms
/// and explicitly declare
/// them.
use crate::models::Charm;

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

/// Importing the structure
/// that models detailed info
/// on a charm.
use crate::responses::CharmDetail;

/// Importing the structure
/// that models detailed info
/// on a user's timeline.
use crate::responses::UserTimeline;

/// Importing the function to retrieve
/// a user by their ID.
use crate::rw_utils::get_user_by_id;

/// Importing the function to retrieve a 
/// user-created charm by its ID to check
/// whether the charm exists and the supplied
/// user can delete this charm.
use crate::rw_utils::get_charm_by_id;

/// Importing the "StatusResponse"
/// structure to return information
/// on operational success.
use crate::responses::StatusResponse;

/// Importing the structure to submit
/// a payload for obtaining a timeline
/// of charms for a user.
use crate::payloads::TimelinePayload;

/// Importing the structure to delete
/// a charm a user created.
use crate::payloads::DeleteCharmPayload;

/// Importing the structure to submit
/// a payload for obtaining detailed
/// info for a charm.
use crate::payloads::CharmDetailPayload;

/// Importing the structure to create
/// a new charm.
use crate::payloads::CreateCharmPayload;

/// Importing the function to retrieve a 
/// user by a token associated with them.
use crate::rw_utils::get_user_from_token;

/// Importing the function to retrieve a 
/// token by a token associated with a user.
use super::rw_utils::get_api_token_object;

/// Attempts to create a new post for a user with the given
/// payload. If this operation succeeds, an instance of 
/// the "Charm" structure is returned. If this operation
/// fails, an error is returned.
pub async fn create_new_charm(
    payload: &CreateCharmPayload,
    pool: &Pool<Postgres>,
) -> Result<Charm, KleahErr> {
    let token: APIToken = match get_api_token_object(&payload.api_token, pool).await {
        Ok(token) => token,
        Err(e) => return Err::<Charm, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let user: KleahUser = match get_user_from_token(&payload.api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Charm, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let str_to_hash: String = format!("{}:{}", get_time(), &payload.charm_text);
    let charm_id: String = match hash(str_to_hash, DEFAULT_COST){
        Ok(charm_id) => charm_id,
        Err(e) => return Err::<Charm, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let file_id: Option<String>;
    match &payload.file_id{
        Some(f) => file_id = Some(f.clone()),
        None => file_id = None
    };
    let mut is_reply: bool = false;
    let refers_to: Option<String>;
    if payload.is_reply{
        is_reply = true;
        refers_to = Some(payload.refers_to.clone());
    }
    else {
        refers_to = None;
    }
    if token.can_post_charms{
        let new_charm: Charm = Charm {
            user_id: user.user_id,
            charm_id: charm_id,
            charm_text: payload.charm_text.clone(),
            created_at: get_time(),
            file_id: file_id,
            is_reply: is_reply,
            refers_to: refers_to,
            reaction_ids: None,
            proclamation_count: None,
            like_count: None,
            reaction_count: None,
        };
        let _insert_op = match query!(
            "INSERT INTO charms (user_id, charm_id, charm_text, created_at, file_id, is_reply, refers_to, reaction_ids, like_count, reaction_count) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
            new_charm.user_id,
            new_charm.charm_id,
            new_charm.charm_text,
            new_charm.created_at,
            new_charm.file_id,
            new_charm.is_reply,
            new_charm.refers_to,
            new_charm.reaction_ids,
            new_charm.like_count,
            new_charm.reaction_count
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<Charm, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(new_charm)
    }
    else {
        let e: String = "Token does not have the correct permissions.".to_string();
        Err::<Charm, KleahErr>(KleahErr::new(&e.to_string()))
    }
}


/// Attempts to delete a charm for a user given 
/// one of their API tokens and charm ID. If this operation 
/// succeeds, an instance of  the "StatusResponse" 
/// structure is returned with a status code of 0. 
/// If this operation fails, an error is returned 
/// or an instance of the "StatusResponse" structure 
/// with the status code of 1 is returned.
pub async fn wipe_charm(
    payload: &DeleteCharmPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr> {
    let token: APIToken = match get_api_token_object(&payload.api_token, pool).await {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let user: KleahUser = match get_user_from_token(&token.token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let charm: Charm = match get_charm_by_id(&payload.charm_id, pool).await {
        Ok(charm) => charm,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if token.can_post_charms && user.user_id == token.user_id && charm.user_id == user.user_id {
        let _wipe_op: () = match query!("DELETE FROM charms WHERE user_id = $1", user.user_id)
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
        let e: String = "Token did not have correct permissions or the charm or user did not exist.".to_string();
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to retrieve a user's charm in detail.
/// If this operation succeeds, an instance of the
/// "CharmDetail" structure is returned. If this operation
/// fails, however, an error is returned.
pub async fn show_charm_detail(
    payload: &CharmDetailPayload,
    pool: &Pool<Postgres>
) -> Result<CharmDetail, KleahErr> {
    let charm: Charm = match get_charm_by_id(&payload.charm_id, pool).await {
        Ok(charm) => charm,
        Err(e) => return Err::<CharmDetail, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let user: KleahUser = match get_user_by_id(&charm.charm_id, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<CharmDetail, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let like_count: i32;
    match charm.like_count {
        Some(refer) => like_count = refer,
        None => like_count = 0
    }
    let reaction_count: i32;
    match charm.reaction_count {
        Some(refer) => reaction_count = refer,
        None => reaction_count = 0
    }
    let result: CharmDetail = CharmDetail{
        username: user.username,
        avatar_url: user.avatar_url,
        created_at: charm.created_at,
        display_name: user.display_name,
        charm_text: charm.charm_text,
        file_url: charm.file_id,
        like_count: like_count,
        reaction_count: reaction_count
    };
    Ok(result)
}

/// Attempts to retrieve a user's timeline.
/// If this operation succeeds, an instance of the
/// "UserTimeline" structure is returned. If this operation
/// fails, however, an error is returned.
pub async fn show_user_timline(
    payload: &TimelinePayload,
    pool: &Pool<Postgres>
) -> Result<UserTimeline, KleahErr> {
    let user: KleahUser = match get_user_from_token(&payload.api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<UserTimeline, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let user_charms: Vec<Charm> = match query_as!(Charm, "SELECT * FROM charms WHERE user_id = $1", user.user_id)
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<UserTimeline, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: Vec<CharmDetail> = Vec::new();
    for charm in user_charms {
        let charm_id: String = charm.charm_id;
        let charm_detail: CharmDetail = match show_charm_detail(&CharmDetailPayload{charm_id: charm_id}, pool).await {
            Ok(charm_detail) => charm_detail,
            Err(e) => return Err::<UserTimeline, KleahErr>(KleahErr::new(&e.to_string()))
        };
        result.push(charm_detail)
    }
    Ok(UserTimeline{charms: result})
}