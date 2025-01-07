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

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::Postgres;

/// Importing this crate's
/// structure to catch and
/// handle errors.
use crate::KleahErr;

/// Importing the "KleahUser"
/// structure to work with users
/// and explicitly declare
/// them.
use crate::models::KleahUser;

/// Importing the "InviteCode"
/// structure to work with
/// creating instance information.
use crate::models::InstanceInfo;

/// Importing the "StatusResponse"
/// structure to return information
/// on operational success.
use crate::responses::StatusResponse;

/// Importing the function to retrieve a 
/// user by a token associated with them.
use crate::rw_utils::get_user_from_token;

/// Importing this structure to submit a payload
/// for updating information in an entity.
use crate::payloads::ChangeEntityPayload;

/// Importing the structure for submitting
/// a payload for creating new invite codes
/// for an instance.
use crate::payloads::CreateInstanceInfoPayload;

/// This function to store a new invite code into
/// the database. If the owner of the supplied API token
/// is the Tianquan, this operation will succeed. An instance
/// of the "InviteCode" structure will be returned. If the 
/// operation fails, an error will be returned.
pub async fn create_instance_info(
    payload: &CreateInstanceInfoPayload,
    pool: &Pool<Postgres>
) -> Result<InstanceInfo, KleahErr> {
    let user: KleahUser = match get_user_from_token(&payload.api_token, &pool).await {
        Ok(user) => user,
        Err(e) => return Err::<InstanceInfo, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user.is_admin && user.is_active{
        let info: InstanceInfo = InstanceInfo{
            instance_id: payload.instance_id.clone(),
            instance_description: payload.instance_description.clone(),
            instance_name: payload.instance_name.clone(),
            kleah_version: payload.kleah_version.clone(),
            admin_user_id: payload.admin_user_id.clone(),
            instance_rules: payload.instance_rules.clone()
        };
        let _insert_op = match query!(
            "INSERT INTO instance_information (instance_id, instance_description, instance_name, kleah_version, admin_user_id, instance_rules) VALUES ($1, $2, $3, $4, $5, $6)",
            info.instance_id,
            info.instance_description,
            info.instance_name,
            info.kleah_version,
            info.admin_user_id,
            info.instance_rules
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<InstanceInfo, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(info)
    }
    else {
        let e: String = "The user with the requesting API token is not an administrator.".to_string();
        Err::<InstanceInfo, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to update the name for the current Kleah instance.
/// If this operation succeeds, an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn update_instance_name(
    payload: &ChangeEntityPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr>{
    let user: KleahUser = match get_user_from_token(&payload.api_token, &pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user.is_admin && user.is_active{
        let _update_name: () = match sqlx::query!("UPDATE instance_information SET instance_name = $1", payload.new_entity)
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(StatusResponse{status:0})
    }
    else {
        let e: String = "The user with the requesting API token is not an administrator.".to_string();
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to update the description for the current Kleah instance.
/// If this operation succeeds, an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn update_instance_description(
    payload: &ChangeEntityPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr>{
    let user: KleahUser = match get_user_from_token(&payload.api_token, &pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user.is_admin && user.is_active{
        let _update_desc: () = match sqlx::query!("UPDATE instance_information SET instance_description = $1", payload.new_entity)
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(StatusResponse{status:0})
    }
    else {
        let e: String = "The user with the requesting API token is not an administrator.".to_string();
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to update the ID for the current Kleah instance.
/// If this operation succeeds, an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn update_instance_id(
    payload: &ChangeEntityPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr>{
    let user: KleahUser = match get_user_from_token(&payload.api_token, &pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user.is_admin && user.is_active{
        let _update_iid: () = match sqlx::query!("UPDATE instance_information SET instance_id = $1", payload.new_entity)
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(StatusResponse{status:0})
    }
    else {
        let e: String = "The user with the requesting API token is not an administrator.".to_string();
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to update the rules for the current Kleah instance.
/// If this operation succeeds, an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn update_instance_rules(
    payload: &ChangeEntityPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr>{
    let user: KleahUser = match get_user_from_token(&payload.api_token, &pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user.is_admin && user.is_active{
        let _update_rules: () = match sqlx::query!("UPDATE instance_information SET instance_rules = $1", payload.new_entity)
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(StatusResponse{status:0})
    }
    else {
        let e: String = "The user with the requesting API token is not an administrator.".to_string();
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}