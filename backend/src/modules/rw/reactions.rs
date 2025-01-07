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
/// to hash strings
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

/// Importing the structure
/// that models the action of a
/// user reacting to a charm.
use crate::models::UserReaction;

/// Importing the "KleahUser"
/// structure to work with users
/// and explicitly declare
/// them.
use crate::models::KleahUser;

/// Importing the structure
/// that models detailed info
/// on a charm.
use crate::responses::CharmDetail;

/// Importing the structure that
/// models all available reactions
/// on an instance.
use crate::models::InstanceReaction;

/// Importing the "StatusResponse"
/// structure to return information
/// on operational success.
use crate::responses::StatusResponse;

/// Importing the function to retrieve a 
/// user-created charm by its ID to check
/// whether the charm exists and the supplied
/// user can delete this charm.
use crate::rw_utils::get_charm_by_id;

/// Importing the function to show
/// a sanitized version of a charm
/// given its ID.
use super::charms::show_charm_detail;

/// Importing the payload for liking
/// and unliking charms made by others.
use crate::payloads::CharmReactPayload;

/// Importing the payload for retrieving
/// a sanitized version of a Charm from
/// its ID.
use crate::payloads::CharmDetailPayload;

/// Importing the function to retrieve a 
/// user by a token associated with them.
use crate::rw_utils::get_user_from_token;

/// Importing the function to get a reaction
/// on an instance by its name.
use crate::rw_utils::get_reaction_by_name;

/// Importing the payload for gathering
/// charms a user has reacted to.
use crate::payloads::ReactedCharmsPayload;

/// Importing the function to retrieve a reaction
/// entry given the user ID and the charm ID.
use crate::rw_utils::get_reaction_from_charm_and_user;

/// Attempts to react to a charm for a user given 
/// one of their API tokens and the charm ID. If this operation 
/// succeeds, an instance of  the "StatusResponse" 
/// structure is returned with a status code of 0. 
/// If this operation fails, an error is returned 
/// or an instance of the "StatusResponse" structure 
/// with the status code of 1 is returned.
pub async fn react_to_charm(
    payload: &CharmReactPayload,
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
    let hashed_time: String = match hash(&format!("{}:{}", &payload.reaction_name,get_time()), DEFAULT_COST){
        Ok(hashed_time) => hashed_time,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let instance_reaction: InstanceReaction= match get_reaction_by_name(&payload.reaction_name, pool).await {
        Ok(instance_reaction) => instance_reaction,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user.is_active{
        let new_reaction: UserReaction = UserReaction{
            reaction_id: hashed_time,
            file_id: instance_reaction.file_id,
            user_id: user.user_id,
            charm_id: charm.charm_id.clone()
        };
        let count: i32;
        match charm.like_count.clone(){
            Some(something) => count = something + 1,
            None => count = 1
        };
        let _insert_op = match sqlx::query!(
            "INSERT INTO user_reactions (reaction_id, user_id, charm_id, file_id) VALUES ($1, $2, $3, $4)",
            new_reaction.reaction_id,
            new_reaction.user_id,
            new_reaction.charm_id,
            new_reaction.file_id
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let _update_like_count: () = match sqlx::query!("UPDATE charms SET reaction_count = $1 WHERE charm_id = $2", Some(count), charm.charm_id)
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

/// Attempts to remove a reaction to a charm for a user given 
/// one of their API tokens and charm ID. If this operation 
/// succeeds, an instance of  the "StatusResponse" 
/// structure is returned with a status code of 0. 
/// If this operation fails, an error is returned 
/// or an instance of the "StatusResponse" structure 
/// with the status code of 1 is returned.
pub async fn remove_reaction_to_charm(
    payload: &CharmReactPayload,
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
    let reaction: UserReaction = match get_reaction_from_charm_and_user(&user.user_id, &charm.charm_id, pool).await {
        Ok(reaction) => reaction,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user.is_active{
        let count: i32;
        match charm.like_count.clone(){
            Some(something) => count = something - 1 as i32,
            None => count = 1 as i32
        };
        let _insert_op = match sqlx::query!(
            "DELETE FROM user_reactions WHERE reaction_id = $1",
            reaction.reaction_id
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

pub async fn get_reacted_charms(
    payload: &ReactedCharmsPayload,
    pool: &Pool<Postgres>
) -> Result<Vec<CharmDetail>, KleahErr> {
    let user: KleahUser = match get_user_from_token(&payload.api_token, &pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Vec<CharmDetail>, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let all_reactions: Vec<UserReaction> = match query_as!(UserReaction, "SELECT * FROM user_reactions")
        .fetch_all(pool)
        .await
    {
        Ok(all_likes) => all_likes,
        Err(e) => return Err::<Vec<CharmDetail>, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: Vec<CharmDetail> = Vec::new();
    for reaction in all_reactions {
        if reaction.user_id == user.user_id{
            let charm_detail_pl: CharmDetailPayload = CharmDetailPayload{
                charm_id: reaction.charm_id
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