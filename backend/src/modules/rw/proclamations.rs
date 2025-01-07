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
/// that models the promotion
/// of a charm by a user.
use crate::models::Proclamation;

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

/// Importing the function to show
/// a sanitized version of a charm
/// given its ID.
use super::charms::show_charm_detail;

/// Importing the payload for retrieving
/// a sanitized version of a Charm from
/// its ID.
use crate::payloads::CharmDetailPayload;

/// Importing the payload for promoting
/// and unpromoting charms made by others.
use crate::payloads::ProclamationPayload;

/// Importing the function to retrieve a 
/// user by a token associated with them.
use crate::rw_utils::get_user_from_token;

/// Importing the payload for gathering
/// charms a user has promoted.
use crate::payloads::ProclaimedCharmsPayload;

/// Importing the function to retrieve a reaction
/// entry given the user ID and the charm ID.
use crate::rw_utils::get_proclamation_from_charm_and_user;

/// Attempts to promote a charm for a user given 
/// one of their API tokens and the charm ID. If this operation 
/// succeeds, an instance of  the "StatusResponse" 
/// structure is returned with a status code of 0. 
/// If this operation fails, an error is returned 
/// or an instance of the "StatusResponse" structure 
/// with the status code of 1 is returned.
pub async fn promote_charm_from_db(
    payload: &ProclamationPayload,
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
    let hashed_time: String = match hash(&format!("{}:{}:{}", &charm.charm_id, get_time(), &charm.charm_text), DEFAULT_COST){
        Ok(hashed_time) => hashed_time,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user.is_active{
        let new_proclamation: Proclamation = Proclamation{
            proclamation_id: hashed_time,
            user_id: user.user_id,
            charm_id: charm.charm_id.clone()
        };
        let count: i32;
        match charm.proclamation_count.clone(){
            Some(something) => count = something + 1,
            None => count = 1
        };
        let _insert_op = match sqlx::query!(
            "INSERT INTO user_proclamations (proclamation_id, user_id, charm_id) VALUES ($1, $2, $3)",
            new_proclamation.proclamation_id,
            new_proclamation.user_id,
            new_proclamation.charm_id,
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

/// Attempts to "unproclaim" a charm for a user given 
/// one of their API tokens and charm ID. If this operation 
/// succeeds, an instance of  the "StatusResponse" 
/// structure is returned with a status code of 0. 
/// If this operation fails, an error is returned 
/// or an instance of the "StatusResponse" structure 
/// with the status code of 1 is returned.
pub async fn remove_proclamation_to_charm(
    payload: &ProclamationPayload,
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
    let proclamation: Proclamation = match get_proclamation_from_charm_and_user(&user.user_id, &charm.charm_id, pool).await {
        Ok(reaction) => reaction,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user.is_active{
        let count: i32;
        match charm.proclamation_count.clone(){
            Some(something) => count = something - 1 as i32,
            None => count = 1 as i32
        };
        let _insert_op = match sqlx::query!(
            "DELETE FROM user_proclamations WHERE proclamation_id = $1",
            proclamation.proclamation_id
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let _update_like_count: () = match sqlx::query!("UPDATE charms SET proclamation_count = $1 WHERE charm_id = $2", Some(count), charm.charm_id)
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

pub async fn get_proclaimed_charms(
    payload: &ProclaimedCharmsPayload,
    pool: &Pool<Postgres>
) -> Result<Vec<CharmDetail>, KleahErr> {
    let user: KleahUser = match get_user_from_token(&payload.api_token, &pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Vec<CharmDetail>, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let all_proclamations: Vec<Proclamation> = match query_as!(Proclamation, "SELECT * FROM user_proclamations")
        .fetch_all(pool)
        .await
    {
        Ok(all_proclamations) => all_proclamations,
        Err(e) => return Err::<Vec<CharmDetail>, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: Vec<CharmDetail> = Vec::new();
    for proclamation in all_proclamations {
        if proclamation.user_id == user.user_id{
            let charm_detail_pl: CharmDetailPayload = CharmDetailPayload{
                charm_id: proclamation.charm_id
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