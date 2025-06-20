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
/// "query" macro
/// from the "sqlx"
/// crate to execute
/// queries.
use sqlx::query;

/// Importing the
/// "query" macro
/// from the "sqlx"
/// crate to execute
/// queries with a 
/// certain return type.
use sqlx::query_as;

/// Importing this crate's
/// error structure.
use crate::err::KleahErr;

/// Importing the model for
/// storing information
/// about an actor.
use crate::models::Actor;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// Importing the model for
/// storing confidential
/// information about an
/// actor.
use crate::models::PrivateActor;

/*
Cleo by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the 
/// "Pool" structure
/// to accept multiple
/// connections to the 
/// database.
use sqlx::Pool;

/// Importing the "query"
/// macro to execute SQL
/// queries to return
/// nothing.
use sqlx::query;

/// Importing the "query_as"
/// macro to execute SQL
/// queries to return
/// something.
use sqlx::query_as;

/// Importing the "Postgres"
/// structure to specify which
/// database one is connecting
/// to.
use sqlx::postgres::Postgres;

/// Importing the "CleoErr"
/// structure to catch and handle
/// errors.
use crate::modules::err::CleoErr;

/// Importing the "CleoUser" model
/// to read and write information
/// about users to and from the 
/// database.
use crate::modules::models::CleoUser;

/// Importing the "hash_string" 
/// function to generate hashes for
/// strings.
use crate::modules::utils::hash_string;

/// Importing the "get_user_from_token"
/// function to retrieve a Cleo
/// user using an API token.
use super::tokens::get_user_from_token;

/// Importing the "InstanceInformation"
/// structure to write information about
/// the instance to the database.
use crate::modules::models::InstanceInformation;

/// Importing the function to retrieve the first
/// record about the current Cleo instance.
use crate::modules::db::general::get_instance_info;

/// This function attempts to
/// retrieve a list of users
/// present on a Cleo instance.
/// If the operation is successful,
/// a vector containing  instances
/// of the "CleoUser" structure is 
/// returned. If the operation fails,
/// an error is returned.
pub async fn get_instance_users(
    api_token: &String,
    pool: &Pool<Postgres>,
) -> Result<Vec<CleoUser>, CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Vec<CleoUser>, CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin {
        let cleo_users: Vec<CleoUser> = match query_as!(
            CleoUser,
            "SELECT * FROM cleo_users WHERE is_admin = $1",
            false
        )   
            .fetch_all(pool)
            .await 
        {
            Ok(cleo_users) => cleo_users,
            Err(e) => return Err::<Vec<CleoUser>, CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(cleo_users)
    }
    else {
        let e: String = format!("User is not an administrator.");
        Err::<Vec<CleoUser>, CleoErr>(CleoErr::new(&e.to_string()))
    }
}

/// This function attempts to
/// retrieve a list of admins
/// present on a Cleo instance.
/// If the operation is successful,
/// a vector containing  instances
/// of the "CleoUser" structure is 
/// returned. If the operation fails,
/// an error is returned.
pub async fn get_instance_admins(
    api_token: &String,
    pool: &Pool<Postgres>,
) -> Result<Vec<CleoUser>, CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Vec<CleoUser>, CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin {
        let cleo_users: Vec<CleoUser> = match query_as!(
            CleoUser,
            "SELECT * FROM cleo_users WHERE is_admin = $1",
            true
        )   
            .fetch_all(pool)
            .await 
        {
            Ok(cleo_users) => cleo_users,
            Err(e) => return Err::<Vec<CleoUser>, CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(cleo_users)
    }
    else {
        let e: String = format!("User is not an administrator.");
        Err::<Vec<CleoUser>, CleoErr>(CleoErr::new(&e.to_string()))
    }
}

/// This function attempts to write
/// instance information to the database.
/// If this operation is successful,
/// a 0 is returned. If this operation fails,
/// an error is returned.
pub async fn create_instance_info(
    pool: &Pool<Postgres>,
    smtp_server: &String,
    hostname: &String,
    instance_name: &String,
    smtp_username: &String,
    smtp_pass: &String,
    file_dir: &String
) -> Result<usize, CleoErr> {
    let hashed_source: String = format!("{}{}", &hostname, &instance_name);
    let instance_id: String = hash_string(&hashed_source);
    let mut result: usize = 1;
    let info_obj: InstanceInformation = InstanceInformation{
        instance_id: instance_id,
        hostname: hostname.to_owned(),
        instance_name: instance_name.to_owned(),
        smtp_server: smtp_server.to_owned(),
        smtp_username: smtp_username.to_owned(),
        smtp_pass: smtp_pass.to_owned(),
        file_dir: file_dir.to_owned()
    };
    let _insert_op = match query!(
        "INSERT INTO instance_info (instance_id, hostname, instance_name, smtp_server, smtp_username, smtp_pass) VALUES ($1, $2, $3, $4, $5, $6)",
        info_obj.instance_id,
        info_obj.hostname,
        info_obj.instance_name,
        info_obj.smtp_server,
        info_obj.smtp_username,
        info_obj.smtp_pass
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => { result = 0 },
        Err(e) => return Err::<usize, CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(result)
}

/// This function attempts to
/// edit the instance's hostname
/// and save this information in the database.
/// If this operation is successful, an empty function
/// is returned. If this operation fails, an error is
/// returned.
pub async fn edit_instance_hostname(
    api_token: &String,
    new_hostname: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let info: InstanceInformation = match get_instance_info(pool).await {
        Ok(info) => info,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin{
        let update_op: () = match query!(
            "UPDATE instance_info SET hostname = $1 WHERE instance_id = $2",
            new_hostname, 
            info.instance_id
        )
            .execute(pool)
            .await 
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(update_op)
    }
    else {
        let e: &str = "The acting user must be an administrator.";
        return Err::<(), CleoErr>(CleoErr::new(&e.to_string()));
    }
}

/// This function attempts to
/// edit the instance's name
/// and save this information in the database.
/// If this operation is successful, an empty function
/// is returned. If this operation fails, an error is
/// returned.
pub async fn edit_instance_name(
    api_token: &String,
    new_name: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let info: InstanceInformation = match get_instance_info(pool).await {
        Ok(info) => info,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin{
        let update_op: () = match query!(
            "UPDATE instance_info SET instance_name = $1 WHERE instance_id = $2",
            new_name, 
            info.instance_id
        )
            .execute(pool)
            .await 
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(update_op)
    }
    else {
        let e: &str = "The acting user must be an administrator.";
        return Err::<(), CleoErr>(CleoErr::new(&e.to_string()));
    }
}

/// This function attempts to edit the
/// username of the SMTP server's user
/// and save this information in the database.
/// If this operation is successful, an empty function
/// is returned. If this operation fails, an error is
/// returned.
pub async fn edit_smtp_username(
    api_token: &String,
    new_name: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let info: InstanceInformation = match get_instance_info(pool).await {
        Ok(info) => info,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin{
        let update_op: () = match query!(
            "UPDATE instance_info SET smtp_username = $1 WHERE instance_id = $2",
            new_name, 
            info.instance_id
        )
            .execute(pool)
            .await 
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(update_op)
    }
    else {
        let e: &str = "The acting user must be an administrator.";
        return Err::<(), CleoErr>(CleoErr::new(&e.to_string()));
    }
}

/// This function attempts to edit the
/// password of the SMTP server's user
/// and save this information in the database.
/// If this operation is successful, an empty function
/// is returned. If this operation fails, an error is
/// returned.
pub async fn edit_smtp_pass(
    api_token: &String,
    new_name: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let info: InstanceInformation = match get_instance_info(pool).await {
        Ok(info) => info,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin{
        let update_op: () = match query!(
            "UPDATE instance_info SET smtp_pass = $1 WHERE instance_id = $2",
            new_name, 
            info.instance_id
        )
            .execute(pool)
            .await 
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(update_op)
    }
    else {
        let e: &str = "The acting user must be an administrator.";
        return Err::<(), CleoErr>(CleoErr::new(&e.to_string()));
    }
}

/// This function attempts to edit the
/// address of the instance's SMTP server
/// and save this information in the database.
/// If this operation is successful, an empty function
/// is returned. If this operation fails, an error is
/// returned.
pub async fn edit_instance_smtp_server(
    api_token: &String,
    new_smtp_server: &String,
    pool: &Pool<Postgres>,
) -> Result<(), CleoErr>{
    let user: CleoUser = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let info: InstanceInformation = match get_instance_info(pool).await {
        Ok(info) => info,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    if user.is_admin{
        let update_op: () = match query!(
            "UPDATE instance_info SET smtp_server = $1 WHERE instance_id = $2",
            new_smtp_server, 
            info.instance_id
        )
            .execute(pool)
            .await 
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
        };
        Ok(update_op)
    }
    else {
        let e: &str = "The acting user must be an administrator.";
        return Err::<(), CleoErr>(CleoErr::new(&e.to_string()));
    }
}
