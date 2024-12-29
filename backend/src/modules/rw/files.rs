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

/// Importing the function to
/// save a byte array into a file.
use crate::utils::save_file;

/// Importing the "KleahUser"
/// structure to work with users
/// and explicitly declare
/// them.
use crate::models::KleahUser;

/// Importing the structure
/// to store files for a user.
use crate::models::KleahUserFile;

/// Importing the function to retrieve a 
/// user by a token associated with them.
use crate::rw_utils::get_user_from_token;

/// This function attempts to store
/// the supplied bytes in a file on disk
/// and writes a new instance of the "KleahUserFile"
/// structure in the database. If this operation fails,
/// an error is returned.
pub async fn store_file(
    file: &Vec<u8>,
    api_token: &String, 
    name: &String,
    pool: &Pool<Postgres>,
    instance_url: &String,
    folder_name: &String,
) -> Result<KleahUserFile, KleahErr>{
    let user: KleahUser = match get_user_from_token(api_token,pool).await {
        Ok(user) => user,
        Err(e) => return Err::<KleahUserFile, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let hashed_id: String = match hash(format!("{}:{}", &name, get_time()), DEFAULT_COST){
        Ok(hashed_id) => hashed_id,
        Err(e) => return Err::<KleahUserFile, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let file_path: String = match save_file(file, &user.username, folder_name, name).await {
        Ok(file_path) => file_path,
        Err(e) => return Err::<KleahUserFile, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let file_url: String = format!("{}/{}", instance_url, file_path);
    let new_file: KleahUserFile = KleahUserFile{
        file_id: hashed_id,
        file_name: name.to_owned(),
        user_id: user.user_id,
        file_path: file_url
    };
    let _insert_op = match sqlx::query!(
        "INSERT INTO user_files (file_id, user_id, file_name, file_path) VALUES ($1, $2, $3, $4)",
        new_file.file_id,
        new_file.user_id,
        new_file.file_name,
        new_file.file_path,
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<KleahUserFile, KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(new_file)
}