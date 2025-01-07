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

/// Importing the macro
/// from the "sqlx" crate
/// to execute SQL queries.
use sqlx::query_as;

/// Importing this crate's
/// structure to catch and
/// handle errors.
use crate::KleahErr;

/// Importing the "Charm"
/// structure to work with charms
/// and explicitly declare
/// them.
use crate::models::KleahUser;

/// Importing the function
/// to create a new user.
use crate::user::write_user;

/// Importing the structure
/// for submitting a payload that
/// allows the administrator to ban a user.
use crate::payloads::BanningPayload;

/// Importing the "StatusResponse"
/// structure to return information
/// on operational success.
use crate::responses::StatusResponse;

/// Importing the structure for the payload
/// to create a user.
use crate::payloads::CreateUserPayload;

/// Importing the structure
/// for submitting a payload that
/// allows the administrator to see
/// all users on their instance and
/// ban users.
use crate::payloads::APITokenOnlyPayload;

/// Importing the function to retrieve a 
/// user by a token associated with them.
use crate::rw_utils::get_user_from_token;

/// Importing the function to retrieve
/// the instance hostname.
use crate::rw_utils::get_instance_hostname;

/// Creates an administrative user when
/// Kleah's backend is started. 
pub async fn create_admin_user(
    admin_username: &String,
    admin_password: &String,
    admin_email: &String,
    host_name: &String,
    smtp_server: &String,
    pool: &Pool<Postgres>
) -> Result<KleahUser, KleahErr> {
    let user_data: CreateUserPayload = CreateUserPayload{
        user_role: "Tianquan".to_string(),
        username: admin_username.to_owned(),
        display_name: "Admin".to_string(),
        avatar_url: "".to_string(),
        banner_url: "".to_string(),
        user_description: "Fill in your description".to_string(),
        email: admin_email.to_owned(),
        pwd: admin_password.to_owned(),
        is_private: false,
        email_token: "".to_string(),
        is_active: false,
        rules_accepted: true,
        is_admin: true
    };
    let new_user: KleahUser = match write_user(&user_data, pool, smtp_server, host_name).await {
        Ok(new_user) => new_user,
        Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(new_user)
}

/// This function attempts to check whether an admin user
/// already exists. If they do, creating a new admin user
/// is not permitted. If any other failures occur, an error
/// is returned.
pub async fn admin_exists(pool: &Pool<Postgres>) -> Result<bool, KleahErr> {
    let mut result: bool = false;
    let all_users: Vec<KleahUser> = match query_as!(KleahUser, "SELECT * FROM users")
        .fetch_all(pool)
        .await
    {
        Ok(all_users) => all_users,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    for user in all_users {
        if user.is_admin == true{ result = true; }
        else {}
    }
    Ok(result)
}

/// This function attempts to get a vector 
/// of all the Kleah users on this instance.
/// If the operation succeeds, this vector is
/// returned. If this operation fails, an error
/// is returned. This action is only available
/// for administrators.
pub async fn get_instance_users(
    payload: &APITokenOnlyPayload, 
    pool: &Pool<Postgres>
) -> Result<Vec<KleahUser>, KleahErr> {
    let user: KleahUser = match get_user_from_token(&payload.api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Vec<KleahUser>, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user.is_admin{
        let hostname: String = match get_instance_hostname(pool).await {
            Ok(hostname) => hostname,
            Err(e) => return Err::<Vec<KleahUser>, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let users: Vec<KleahUser> = match query_as!(KleahUser, "SELECT * FROM users WHERE host = $1", hostname)
            .fetch_all(pool)
            .await
        {
            Ok(users) => users,
            Err(e) => return Err::<Vec<KleahUser>, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(users)
    }
    else {
        let e: &str = "The requesting user must be an administrator.";
        Err::<Vec<KleahUser>, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// This function attempts to ban a user if
/// the requesting user is an administrator.
/// If the operation succeeds an instance of the
/// "StatusResponse" structure is returned with
/// the status code of 0. If this operation fails,
/// an error is returned. This action is only available
/// for administrators.
pub async fn ban_user(
    payload: &BanningPayload, 
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr> {
    let user: KleahUser = match get_user_from_token(&payload.api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user.is_admin{
        let hostname: String = match get_instance_hostname(pool).await {
            Ok(hostname) => hostname,
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let users: Vec<KleahUser> = match query_as!(KleahUser, "SELECT * FROM users WHERE host = $1", hostname)
            .fetch_all(pool)
            .await
        {
            Ok(users) => users,
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let mut result: usize = 1;
        for user in users {
            if user.user_id == payload.target{
                let _ban_op: () = match query!("DELETE FROM users WHERE user_id = $1", &payload.target)
                    .execute(pool)
                    .await
                {
                    Ok(_ban_op) => {},
                    Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
                };
                result = 0;
            }
        }
        Ok(StatusResponse{status:result})
    }
    else {
        let e: &str = "The requesting user must be an administrator.";
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}
