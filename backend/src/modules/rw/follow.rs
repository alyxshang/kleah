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

/// Importing the 
/// "hash" function
/// to hash strings.
use bcrypt::hash;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::Postgres;

/// Importing the macro
/// from the "sqlx" crate
/// to execute SQL queries.
use sqlx::query_as;

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

/// Importing the structure that models
/// a follower relationship between two
/// Kleah users.
use crate::models::KleahUserFollows;

/// Importing the function to retrieve
/// a user by their ID.
use crate::rw_utils::get_user_by_id;

/// Importing the "StatusResponse"
/// structure to return information
/// on operational success.
use crate::responses::StatusResponse;

/// Importing the structure for submitting
/// a payload for understanding user
/// relationships.
use crate::payloads::UserViewPayload;

/// Importing the payload for following
/// or unfollowing a user.
use crate::payloads::UserInteractionPayload;

/// Importing the function to retrieve a 
/// user by a token associated with them.
use crate::rw_utils::get_user_from_token;

/// Importing the function to retrieve an 
/// API token for a user.
use crate::rw_utils::get_api_token_object;

/// Attempts to follow a user with 
/// the supplied username. If the operation
/// is successful, an instance of the "StatusResponse"
/// structure is returned with a status code of "0".
/// If this operation fails, an error is returned.
pub async fn write_follow_user(
    payload: UserInteractionPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr> {
    let user_who_is_following: KleahUser = match get_user_from_token(&payload.api_token, pool).await {
        Ok(user_who_is_following) => user_who_is_following,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let token_obj: APIToken = match get_api_token_object(&payload.api_token, pool).await {
        Ok(token_obj) => token_obj,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let user_to_follow: KleahUser = match get_user_by_id(&payload.receiver_id, pool).await {
        Ok(user_to_follow) => user_to_follow,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let rel_id: String = match hash(&format!("{}{}{}", get_time(), user_who_is_following.user_id, user_to_follow.user_id), DEFAULT_COST){
        Ok(rel_id) => rel_id,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if token_obj.is_active && token_obj.user_id == user_who_is_following.user_id {
        let follow_rel: KleahUserFollows = KleahUserFollows{
            follower: user_who_is_following.user_id,
            followee: user_to_follow.user_id,
            relationship_id: rel_id
        };
        let _insert_op = match sqlx::query!(
            "INSERT INTO user_follows (follower, followee, relationship_id) VALUES ($1, $2, $3)",
            follow_rel.follower,
            follow_rel.followee,
            follow_rel.relationship_id
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(StatusResponse{ status: 0})
    }
    else {
        let e: String = "Token does not have the correct permissions or users do not exist.".to_string();
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to unfollow a user with 
/// the supplied username. If the operation
/// is successful, an instance of the "StatusResponse"
/// structure is returned with a status code of "0".
/// If this operation fails, an error is returned.
pub async fn write_unfollow_user(
    payload: &UserInteractionPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr> {
    let user_who_is_following: KleahUser = match get_user_from_token(&payload.api_token, pool).await {
        Ok(user_who_is_following) => user_who_is_following,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let _wipe_op: () = match sqlx::query!("DELETE FROM user_follows WHERE follower = $1", user_who_is_following.user_id)
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let status: StatusResponse = StatusResponse{ status: 0 };
    Ok(status)
}

/// Checks whether the owner of the submitting
/// API token in the payload is a follower. If this
/// is the case, a boolean "true" is returned. If it is
/// not, a boolean "false" is returned.
pub async fn user_is_following(
    payload: &UserViewPayload,
    pool: &Pool<Postgres>
) -> Result<bool, KleahErr>{
    let follows: Vec<KleahUserFollows> = match query_as!(KleahUserFollows, "SELECT * FROM user_follows")
        .fetch_all(pool)
        .await
    {
        Ok(blocks) => blocks,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let asker: KleahUser = match get_user_from_token(&payload.api_token, pool).await {
        Ok(asker) => asker,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: bool = false;
    for follow in follows {
        if follow.follower == asker.user_id && follow.followee == payload.issuer{
            result = true;
        }
        else {}
    }
    Ok(result)
}