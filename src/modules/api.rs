/*
Jade by Alyx Shang.
Licensed under the FSL v1.
*/

use std::io::Read;

/// Importing the
/// "post" macro to
/// receieve a post 
/// request.
use actix_web::post;

use crate::JadeUserFile;

/// Importing this crate's
/// error structure.
use super::err::JadeErr;

/// Importing the function
/// to delete a user from
/// the database.
use super::rw::wipe_user;

/// Importing the "Data"
/// structure from "actix-web"
/// to main a connection
/// to the database.
use actix_web::web::Data;

/// Importing the function
/// to delete a user's mood
/// from the database.
use super::rw::wipe_mood;

/// Importing the "Json"
/// structure so Jade
/// can return JSON responses.
use actix_web::web::Json;

/// Importing the "Path"
/// structure to extract
/// an email verification
/// token.
use actix_web::web::Path;

/// Importing the function
/// to store files in the
/// database.
use super::rw::store_file;

/// Importing the function
/// to delete a user's API
/// token from the database.
use super::rw::wipe_token;

/// Importing the function
/// to create a new Jade
/// user.
use super::rw::write_user;

/// Importing the "AppData"
/// structure to maintain
/// a connection to the
/// database.
use super::units::AppData;

/// Importing the "APIToken"
/// structure for explicit
/// typing.
use super::units::APIToken;


/// Importing the "JadeMood"
/// structure for explicit
/// typing.
use super::units::JadeMood;

/// Importing the "JadeUser"
/// structure for explicit
/// typing.
use super::units::JadeUser;

/// Importing this
/// structure to send
/// HTTP responses.
use actix_web::HttpResponse;

/// Importing the function
/// to retrieve a users mood.
use super::rw::get_user_mood;

/// Importing the function
/// to retrieve all moods a 
/// user has ever posted.
use super::rw::get_user_moods;

/// Importing the function
/// to retrieve all moods a 
/// user has ever used.
use super::rw::get_user_tokens;

/// Importing the function
/// to create a new mood for
/// a user.
use super::rw::create_new_mood;

/// Importing the function
/// to create a new token for
/// a user.
use super::rw::create_new_token;

/// Importing the function
/// to create a update a user's
/// email address.
use super::rw::update_user_email;

/// Importing the structure that emulates
/// a form for uploading files.
use super::units::FileUploadForm;

/// Importing the structure
/// to return information
/// about the status of an operation.
use super::units::StatusResponse;

/// Importing a function to
/// verify a user's email address.
use super::rw::verify_user_email;

/// Importing the structure
/// for routes that only require an
/// API token.
use super::units::TokenOnlyPayload;

/// Importing the structure
/// for routes that only offer
/// operations for managing moods.
use super::units::MoodActionPayload;

/// Importing the function
/// to create a update a user's
/// password.
use super::rw::update_user_password;

/// Importing the structure
/// to return information on all
/// moods a user has ever posted.
use super::units::UserMoodsResponse;

/// Importing the structure
/// that helps submit information
/// for creating new users.
use super::units::CreateUserPayload;

/// Importing the payload to 
/// wipe tokens.
use super::units::DeleteTokenPayload;

/// Importing the structure
/// that helps submit information
/// for creating new API tokens.
use super::units::CreateTokenPayload;

/// Importing the structure
/// for routes that only require the
/// user's username.
use super::units::UsernameOnlyPayload;

/// Importing the structure
/// for routes that only modify
/// a user's information.
use super::units::ChangeEntityPayload;

/// Importing the structure
/// for routes that only concern
/// a user's API tokens.
use super::units::UserAPITokensPayload;

/// Importing the trait to make
/// multipart file uploads.
use actix_multipart::form::MultipartForm;

/// Importing the structure to return
/// information on whether email address
/// verification was successful or not.
use super::units::EmailVerificationStatus;

/// This function contains the API service
/// to upload user files and return data on this
/// file as an instance of the "JadeUserFile" structure.
/// If the operation fails, an error is returned.
#[post("files/upload")]
pub async fn upload_user_file(
    MultipartForm(form): MultipartForm<FileUploadForm>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr>{
    let mut buf: Vec<u8> = Vec::new();
    let _read_op: usize = match form.file.file.as_file().read_to_end(&mut buf){
        Ok(_read_op) => _read_op,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let user_file: JadeUserFile = match store_file(
        &buf, 
        &form.metadata.api_token, 
        &form.metadata.name, 
        &data.pool,
        &data.instance_url,
        &data.file_folder
    ).await {
        Ok(user_file) => user_file,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(user_file))
}

#[post("/email/verify/{email_token}")]
pub async fn verify_email(
    token: Path<String>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let verified: bool = match verify_user_email(&token, &data.pool).await {
        Ok(verified) => verified,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let res: EmailVerificationStatus = EmailVerificationStatus{status:verified};
    Ok(HttpResponse::Ok().json(res))
}

/// This API route attempts to create a new user
/// with the given payload. If this operation
/// fails, an error response is returend.
pub async fn create_user(
    payload: Json<CreateUserPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let created: JadeUser = match write_user(&payload, &data.pool, &data.smtp_server).await {
        Ok(created) => created,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(created))
}

/// This API route attempts to delete a user
/// with the given payload. If this operation
/// fails, an error response is returend.
pub async fn delete_user(
    payload: Json<TokenOnlyPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let wiped: StatusResponse = match wipe_user(&payload, &data.pool).await {
        Ok(created) => created,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(wiped))
}

/// This API route attempts to create a new API
/// token with the given payload. If this operation
/// fails, an error response is returend.
pub async fn create_token(
    payload: Json<CreateTokenPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let wiped: APIToken = match create_new_token(&payload, &data.pool).await {
        Ok(created) => created,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(wiped))
}

/// This API route attempts to delete an API
/// token with the given payload. If this operation
/// fails, an error response is returend.
pub async fn delete_token(
    payload: Json<DeleteTokenPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let wiped: StatusResponse = match wipe_token(&payload, &data.pool).await {
        Ok(wiped) => wiped,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(wiped))
}

/// This API route attempts to create a new mood
/// with the given payload. If this operation
/// fails, an error response is returend.
pub async fn set_mood(
    payload: Json<MoodActionPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let new_mood: JadeMood = match create_new_mood(&payload, &data.pool).await {
        Ok(new_mood) => new_mood,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(new_mood))
}

/// This API route attempts to delete a mood
/// with the given payload. If this operation
/// fails, an error response is returend.
pub async fn delete_mood(
    payload: Json<MoodActionPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let status: StatusResponse = match wipe_mood(&payload, &data.pool).await {
        Ok(status) => status,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(status))
}

/// This API route attempts to change a user's
/// password with the given payload. 
/// If this operation fails, an error 
/// response is returend.
pub async fn change_user_pwd(
    payload: Json<ChangeEntityPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let op_status: StatusResponse = match update_user_password(&payload, &data.pool).await {
        Ok(op_status) => op_status,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(op_status))
}

/// This API route attempts to change a user's
/// email with the given payload. 
/// If this operation fails, an error 
/// response is returend.
pub async fn change_user_email(
    payload: Json<ChangeEntityPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let op_status: StatusResponse = match update_user_email(&payload, &data.pool, &data.smtp_server).await {
        Ok(op_status) => op_status,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(op_status))
}

/// This API route attempts to get a user's
/// mood with the given payload. 
/// If this operation fails, an error 
/// response is returend.
pub async fn get_mood(
    payload: Json<UsernameOnlyPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let mood: JadeMood = match get_user_mood(&payload, &data.pool).await {
        Ok(mood) => mood,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(mood))
}

/// This API route attempts to get a user's
/// moods with the given payload. 
/// If this operation fails, an error 
/// response is returend.
pub async fn get_moods(
    payload: Json<UsernameOnlyPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let moods: UserMoodsResponse = match get_user_moods(&payload, &data.pool).await {
        Ok(moods) => moods,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(moods))
}

/// This API route attempts to get a user's
/// tokens with the given payload. 
/// If this operation fails, an error 
/// response is returend.
pub async fn get_tokens(
    payload: Json<UserAPITokensPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, JadeErr> {
    let tokens: Vec<APIToken> = match get_user_tokens(&payload, &data.pool).await {
        Ok(tokens) => tokens,
        Err(e) => return Err::<HttpResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(HttpResponse::Ok().json(tokens))
}