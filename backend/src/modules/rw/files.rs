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

/// Importing the function to
/// save a byte array into a file.
use crate::utils::save_file;

/// Importing the "KleahUser"
/// structure to work with users
/// and explicitly declare
/// them.
use crate::models::KleahUser;

/// Importing the function to
/// delete a user-uploaded file
/// from disk.
use crate::utils::delete_file;

/// Importing the structure
/// to store files for a user.
use crate::models::KleahUserFile;

/// Importing the function to retrieve
/// a user by their ID.
use crate::rw_utils::get_user_by_id;

/// Importing the function to 
/// retrieve a file by its ID.
use super::rw_utils::get_file_by_id;

/// Importing the structure that allows
/// one to submit a payload for retrieving
/// profile information about a user.
use crate::payloads::ProfilePayload;

/// Importing the "StatusResponse"
/// structure to return information
/// on operational success.
use crate::responses::StatusResponse;

/// Importing the structure for the payload
/// to delete a 
/// user-uploaded file.
use crate::payloads::DeleteFilePayload;

/// Importing the response for showing 
/// a user's files.
use crate::responses::UserFilesTimeline;

/// Importing the function to retrieve a 
/// user by a token associated with them.
use crate::rw_utils::get_user_from_token;

/// Importing the structure for the payload
/// to change the privacy perms for a 
/// user-uploaded file.
use crate::payloads::UpdateFilePermsPayload;

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
    folder_name: &String,
    is_private: &bool
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
    let new_file: KleahUserFile = KleahUserFile{
        file_id: hashed_id,
        file_name: name.to_owned(),
        user_id: user.user_id,
        file_path: file_path,
        is_private: is_private.to_owned()
    };
    let _insert_op = match sqlx::query!(
        "INSERT INTO user_files (file_id, user_id, file_name, file_path, is_private) VALUES ($1, $2, $3, $4, $5)",
        new_file.file_id,
        new_file.user_id,
        new_file.file_name,
        new_file.file_path,
        new_file.is_private
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<KleahUserFile, KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(new_file)
}

/// Attempts to update the privacy paramater on 
/// a user-uploaded file. If this operation succeeds, 
/// an instance of  the "StatusResponse" structure 
/// is returned with a status code of 0. 
/// If this operation fails, an error is returned 
/// or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn update_file_perms(
    payload: &UpdateFilePermsPayload,
    pool: &Pool<Postgres>,
) -> Result<StatusResponse, KleahErr>{
    let token: APIToken = match query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let user: KleahUser = match get_user_by_id(&token.user_id, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let file_subject: KleahUserFile = match get_file_by_id(&payload.file_id, pool).await {
        Ok(file_subject) => file_subject,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if token.is_active && token.user_id == user.user_id && file_subject.user_id == user.user_id
    {
        let _update_op: () = match query!("UPDATE user_files SET is_private = $1 WHERE user_id = $2", payload.is_private, payload.file_id)
            .execute(pool)
            .await
        {

            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(StatusResponse{status: 0})
    }
    else {
        let e: String = "Token does not have the correct permissions".to_string();
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to delete a file
/// a user uploaded. If this operation succeeds, 
/// an instance of  the "StatusResponse" structure 
/// is returned with a status code of 0. 
/// If this operation fails, an error is returned 
/// or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn delete_user_file_from_db(
    payload: &DeleteFilePayload,
    pool: &Pool<Postgres>,
) -> Result<StatusResponse, KleahErr> {
    let token: APIToken = match query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let user: KleahUser = match get_user_by_id(&token.user_id, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let file_subject: KleahUserFile = match get_file_by_id(&payload.file_id, pool).await {
        Ok(file_subject) => file_subject,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if token.is_active && 
       token.user_id == user.user_id &&
       file_subject.user_id == user.user_id
    {
        let _update_op: () = match query!("DELETE FROM user_files WHERE file_id = $1", payload.file_id)
            .execute(pool)
            .await
        {

            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let deleted: bool = match delete_file(&file_subject.file_path).await {
            Ok(deleted) => deleted,
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        if deleted{
            Ok(StatusResponse{status: 0})
        }
        else {
            let e: String = "Could not delete file from disk.".to_string();
            Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        }
    }
    else {
        let e: String = "Token does not have the correct permissions or the file does not exist.".to_string();
        Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

pub async fn assemble_files(
    payload: &ProfilePayload,
    pool: &Pool<Postgres>
) -> Result<UserFilesTimeline, KleahErr> {
    let user: KleahUser = match get_user_from_token(&payload.api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<UserFilesTimeline, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user.is_active{
        let files: Vec<KleahUserFile> = match query_as!(KleahUserFile, "SELECT * FROM user_files")
            .fetch_all(pool)
            .await
        {
            Ok(files) => files,
            Err(e) => return Err::<UserFilesTimeline, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let mut result: Vec<KleahUserFile> = Vec::new();
        for file in files {
            if file.user_id == user.user_id {
                result.push(file);
            }
            else {}
        }
        Ok(UserFilesTimeline{files:result})
    }
    else {
        let e: String = format!("User \"{}\" needs to be verified to have a public profile.", &user.username);
        Err::<UserFilesTimeline, KleahErr>(KleahErr::new(&e.to_string()))
    }
}