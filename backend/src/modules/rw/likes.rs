/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

use sqlx::query_as;
/// Importing the
/// "Pool" structure
/// from the "sqlx" crate
/// to make a pool for
/// database connections.
use sqlx::Pool;

/// Importing the 
/// "hash" function
/// to hash strings
use bcrypt::hash;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::Postgres;

use crate::payloads::CharmDetailPayload;
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

/// Importing the structure
/// that models the action of a
/// user liking a charm.
use crate::models::UserLike;

/// Importing the "KleahUser"
/// structure to work with users
/// and explicitly declare
/// them.
use crate::models::KleahUser;

/// Importing the structure
/// that models detailed info
/// on a charm.
use crate::responses::CharmDetail;

/// Importing the "StatusResponse"
/// structure to return information
/// on operational success.
use crate::responses::StatusResponse;

/// Importing the function to retrieve a 
/// user-created charm by its ID to check
/// whether the charm exists and the supplied
/// user can delete this charm.
use crate::rw_utils::get_charm_by_id;

/// Importing the payload for liking
/// and unliking charms made by others.
use crate::payloads::CharmLikePayload;

/// Importing the payload for gathering
/// all charms a user has liked.
use crate::payloads::LikedCharmsPayload;

/// Importing the function to retrieve a 
/// user by a token associated with them.
use crate::rw_utils::get_user_from_token;

/// Importing the function to retrieve a like
/// entry given the user ID and the charm ID.
use crate::rw_utils::get_like_from_charm_and_user;

use super::charms::show_charm_detail;

/// Attempts to like a charm for a user given 
/// one of their API tokens and charm ID. If this operation 
/// succeeds, an instance of  the "StatusResponse" 
/// structure is returned with a status code of 0. 
/// If this operation fails, an error is returned 
/// or an instance of the "StatusResponse" structure 
/// with the status code of 1 is returned.
pub async fn like_charm(
    payload: &CharmLikePayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr> {
    let user: KleahUser = match get_user_from_token(&payload.api_token, &pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let charm: Charm = match get_charm_by_id(&payload.charm_id, &pool).await {
        Ok(charm) => charm,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let hashed_time: String = match hash(get_time(), DEFAULT_COST){
        Ok(hashed_time) => hashed_time,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user.is_active{
        let new_like: UserLike = UserLike{
            like_id: hashed_time,
            user_id: user.user_id,
            charm_id: charm.charm_id.clone()
        };
        let count: i32;
        match charm.like_count.clone(){
            Some(something) => count = something + 1,
            None => count = 1
        };
        let _insert_op = match sqlx::query!(
            "INSERT INTO user_likes (like_id, user_id, charm_id) VALUES ($1, $2, $3)",
            new_like.like_id,
            new_like.user_id,
            new_like.charm_id
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let _update_like_count: () = match sqlx::query!("UPDATE charms SET like_count = $1 WHERE charm_id = $2", Some(count), charm.charm_id)
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(StatusResponse{status:0})
    }
    else {
        let e: String = "The user must verify their email address.".to_string();
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to unlike a charm for a user given 
/// one of their API tokens and charm ID. If this operation 
/// succeeds, an instance of  the "StatusResponse" 
/// structure is returned with a status code of 0. 
/// If this operation fails, an error is returned 
/// or an instance of the "StatusResponse" structure 
/// with the status code of 1 is returned.
pub async fn unlike_charm(
    payload: &CharmLikePayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr> {
    let user: KleahUser = match get_user_from_token(&payload.api_token, &pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let charm: Charm = match get_charm_by_id(&payload.charm_id, &pool).await {
        Ok(charm) => charm,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let like: UserLike = match get_like_from_charm_and_user(&user.user_id, &charm.charm_id, pool).await {
        Ok(like) => like,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user.is_active{
        let count: i32;
        match charm.like_count.clone(){
            Some(something) => count = something - 1 as i32,
            None => count = 1 as i32
        };
        let _insert_op = match sqlx::query!(
            "DELETE FROM user_likes WHERE like_id = $1",
            like.like_id
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let _update_like_count: () = match sqlx::query!("UPDATE charms SET like_count = $1 WHERE charm_id = $2", Some(count), charm.charm_id)
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(StatusResponse{status:0})
    }
    else {
        let e: String = "The user must verify their email address.".to_string();
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

pub async fn get_liked_charms(
    payload: &LikedCharmsPayload,
    pool: &Pool<Postgres>
) -> Result<Vec<CharmDetail>, KleahErr> {
    let user: KleahUser = match get_user_from_token(&payload.api_token, &pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Vec<CharmDetail>, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let all_likes: Vec<UserLike> = match query_as!(UserLike, "SELECT * FROM user_likes").fetch_all(pool)
        .await
    {
        Ok(all_likes) => all_likes,
        Err(e) => return Err::<Vec<CharmDetail>, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: Vec<CharmDetail> = Vec::new();
    for like in all_likes {
        if like.user_id == user.user_id{
            let charm_detail_pl: CharmDetailPayload = CharmDetailPayload{
                charm_id: like.charm_id
            };
            let charm_detail: CharmDetail = match show_charm_detail(&charm_detail_pl, pool).await {
                Ok(charm_detail) => charm_detail,
                Err(e) => return Err::<Vec<CharmDetail>, KleahErr>(KleahErr::new(&e.to_string()))
            };
            result.push(charm_detail);
        }
        else {}
    }
    Ok(result)
}