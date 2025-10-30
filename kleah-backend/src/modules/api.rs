/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the function
/// to verify a hashed string.
use bcrypt::verify;

/// Importing the function
/// macro for making "POST"
/// requests.
use actix_web::post;

/// Importing the structure
/// for catching and handling
/// errors.
use super::err::KleahErr;

/// Importing the structure to
/// accept JSON payloads as an
/// argument for a service function.
use actix_web::web::Json;

/// Importing the structure
/// for maintaining a persistent
/// state of data at app runtime.
use actix_web::web::Data;

/// Importing the data structure
/// containing the app-wide
/// pool of connections to the
/// database.
use super::units::AppData;

/// Importing the function
/// to check whether a user
/// exists or not.
use super::db::user_exists;

/// Importing the function to
/// change the name of a Kleah
/// user and the corresponding
/// actor.
use super::db::update_name;

/// Importing the function to
/// update the email of a
/// Kleah user.
use super::db::update_email;

/// Importing the "Httpresponse"
/// structure to return errors
/// as HTTP responses.
use actix_web::HttpResponse;

/// Importing the data structure
/// modelling data about a Kleah user
/// in the database.
use super::models::KleahUser;

/// Importing the function to wipe
/// a record of an API token from
/// the database.
use super::db::destroy_token;

/// Importing the data structure
/// modelling data about an ActivityPub
/// actor on a Kleah instance in the 
/// database.
use super::models::KleahActor;

/// Importing the function to
/// retrieve the record of a user
/// in the database given their
/// username.
use super::db::get_user_by_id;

/// Importing the function to check
/// whether a supplied string is a
/// valid email address.
use super::utils::check_email;

/// Importing the function to
/// create a record for a new
/// Kleah user in the database.
use super::db::create_new_user;

/// Importing the function to
/// update the password of a
/// Kleah user.
use super::db::update_password;

/// Importing the function to
/// create a record for a new
/// Kleah ActivityPub actor
/// in the database.
use super::db::create_new_actor;

/// Importing the function to create
/// a new record for a new API token
/// for a Kleah user in the database.
use super::db::create_api_token;

/// Importing the data structure
/// modelling a user's API token
/// in the database.
use super::models::UserAPIToken;


/// Importing the function for
/// retrieving information about
/// the current Kleah instance.
use super::db::get_instance_info;

/// Importing the function to 
/// retrieve the record of a user
/// given that user's API token.
use super::db::get_user_by_token;

/// Importing the function to check
/// whether a supplied string is a
/// valid username.
use super::utils::check_username;

/// Importing the function to check
/// whether a supplied string is a
/// valid password.
use super::utils::check_password;

/// Importing the function to
/// change the name of a Kleah
/// user and the corresponding
/// actor.
use super::db::update_description;

/// Importing the enumeration describing
/// the types of Kleah users that can
/// exist.
use super::payloads::KleahUserType;

/// Importing the structure for serializing 
/// a Rust data structure containing data
/// on whether a change to a user's record
/// was successful or not into
/// a JSON string.
use super::responses::StatusResponse;

/// Importing the data structure
/// modelling data about the current
/// Kleah instance in the database.
use super::models::InstanceInformation;

/// Importing the structure representing
/// a JSON payload containing data for 
/// creating a new Kleah user.
use super::payloads::UserCreatePayload;

/// Importing the data structure representing
/// a JSON payload containing data to make a
/// trivial change to the record(s) of a user
/// in the database.
use super::payloads::UserChangePayload;

/// Importing the structure representing
/// a JSON payload containing data for 
/// creating a new API token for a Kleah user.
use super::payloads::CreateTokenPayload;

/// Importing the structure for serializing 
/// a Rust data structure containing data
/// on a created Kleah user and actor into
/// a JSON string.
use super::responses::UserCreateResponse;

/// Importing the structure for serializing 
/// a Rust data structure containing data
/// on a created API token for a Kleah user
/// into a JSON string.
use super::responses::CreateTokenResponse;

/// Importing the data structure representing
/// a JSON payload containing data to make a
/// signifcant change to the record(s) of a user
/// in the database.
use super::payloads::SecureUserChangePayload;

/// A service function that accepts a JSON
/// payload for creating a new Kleah user
/// and a new Kleah ActivityPub actor. If both
/// these operation are successful, an HTTP 
/// response is returned with information on
/// the created user. If the operation fails,
/// an error is sent.
#[post("/api/user/create")]
pub async fn create_user_service(
    payload: Json<UserCreatePayload>,
    data: Data<AppData>
) -> Result<HttpResponse, KleahErr>{
    let u_e: bool = user_exists(&payload.username, &data.pool).await;
    if check_username(&payload.username) &&
       check_password(&payload.password) &&
       check_email(&payload.email_addr) &&
       !u_e
    {
        let kleah_user: KleahUser = match create_new_user(
            &payload.name,
            &payload.password,
            &payload.username,
            &payload.email_addr,
            &payload.description,
            &false,
            &data.pool
        ).await {
            Ok(kleah_user) => kleah_user,
            Err(e) => return Err::<HttpResponse, KleahErr>(
                KleahErr::new(&e.to_string())
            )
        };
        let instance: InstanceInformation = match get_instance_info(
            &data.pool
        ).await{
            Ok(instance) => instance,
            Err(e) => return Err::<HttpResponse, KleahErr>(
                KleahErr::new(&e.to_string())
            )
        };
        let actor_type: String = match payload.user_type{
            KleahUserType::Person => "Person".to_string(),
            KleahUserType::Bot => "Bot".to_string()
        };
        let liked_endpoint: String = format!("/apub/{}/liked", &kleah_user.username);
        let inbox_endpoint: String = format!("/apub/{}/inbox", &kleah_user.username);
        let outbox_endpoint: String = format!("/apub/{}/outbox", &kleah_user.username);
        let following_endpoint: String = format!("/apub/{}/following", &kleah_user.username);
        let followers_endpoint: String = format!("/apub/{}/followers", &kleah_user.username);
        let pubkey_endpoint: String = format!("/apub/{}/pubkey", &kleah_user.username);
        let kleah_actor: KleahActor = match create_new_actor(
            &kleah_user.name,
            &instance.host,
            &kleah_user.username,
            &actor_type,
            &kleah_user.description,
            &liked_endpoint,
            &inbox_endpoint,
            &outbox_endpoint,
            &following_endpoint,
            &followers_endpoint,
            &pubkey_endpoint,
            &data.pool
        ).await {
            Ok(kleah_actor) => kleah_actor,
            Err(e) => return Err::<HttpResponse, KleahErr>(
                KleahErr::new(&e.to_string())
            )
        };
        let resp: UserCreateResponse = UserCreateResponse{
            name: kleah_actor.name,
            username: kleah_actor.username,
            description: kleah_actor.description
        };
        Ok(HttpResponse::Ok().json(resp))
    }
    else {
        Err::<HttpResponse, KleahErr>(
            KleahErr::new("Password, E-Mail address or username were of the wrong format.")
        )
    }
}

#[post("/api/user/edit/name")]
pub async fn update_name_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> Result<HttpResponse, KleahErr>{
    let user: KleahUser = match get_user_by_token(
        &payload.api_token,
        &data.pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<HttpResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let status: bool = match update_name(
        &user.username,
        &payload.new_entity,
        &data.pool
    ).await {
        Ok(_f) => true,
        Err(_e) => false
    };
    let resp: StatusResponse = StatusResponse{
        status: status
    };
    Ok(HttpResponse::Ok().json(resp))
}

#[post("/api/user/edit/bio")]
pub async fn update_description_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> Result<HttpResponse, KleahErr>{
    let user: KleahUser = match get_user_by_token(
        &payload.api_token,
        &data.pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<HttpResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let status: bool = match update_description(
        &user.username,
        &payload.new_entity,
        &data.pool
    ).await {
        Ok(_f) => true,
        Err(_e) => false
    };
    let resp: StatusResponse = StatusResponse{
        status: status
    };
    Ok(HttpResponse::Ok().json(resp))
}

#[post("/api/user/edit/password")]
pub async fn update_password_service(
    payload: Json<SecureUserChangePayload>,
    data: Data<AppData>
) -> Result<HttpResponse, KleahErr>{
    let user: KleahUser = match get_user_by_token(
        &payload.api_token,
        &data.pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<HttpResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let verified: bool = match verify(
        &payload.old_entity,
        &user.password
    ){
        Ok(verified) => verified,
        Err(e) => return Err::<HttpResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    if verified {
        let status: bool = match update_password(
           &user.username,
           &payload.new_entity,
           &data.pool
        ).await {
            Ok(_f) => true,
            Err(_e) => false
        };
        let resp: StatusResponse = StatusResponse{
            status: status
        };
        Ok(HttpResponse::Ok().json(resp))
    }
    else {
        Err::<HttpResponse,KleahErr>(
            KleahErr::new("Password integrity could not be verified.")
        )
    }
}

#[post("/api/user/edit/email")]
pub async fn update_email_service(
    payload: Json<SecureUserChangePayload>,
    data: Data<AppData>
) -> Result<HttpResponse, KleahErr>{
    let user: KleahUser = match get_user_by_token(
        &payload.api_token,
        &data.pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<HttpResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let verified: bool = match verify(
        &payload.old_entity,
        &user.email_addr
    ){
        Ok(verified) => verified,
        Err(e) => return Err::<HttpResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    if verified {
        let status: bool = match update_email(
           &user.username,
           &payload.new_entity,
           &data.pool
        ).await {
            Ok(_f) => true,
            Err(_e) => false
        };
        let resp: StatusResponse = StatusResponse{
            status: status
        };
        Ok(HttpResponse::Ok().json(resp))
    }
    else {
        Err::<HttpResponse,KleahErr>(
            KleahErr::new("Email integrity could not be verified.")
        )
    }
}

#[post("/api/user/token/create")]
pub async fn create_api_token_service(
    payload: Json<CreateTokenPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, KleahErr>{
    let user: KleahUser = match get_user_by_id(
        &payload.username,
        &data.pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<HttpResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let verified: bool = match verify(
        &payload.password,
        &user.password
    ){
        Ok(verified) => verified,
        Err(e) => return Err::<HttpResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    if verified {
        let token: UserAPIToken = match create_api_token(
            &user.username,
            &data.pool
        ).await {
            Ok(token) => token,
            Err(e) => return Err::<HttpResponse, KleahErr>(
                KleahErr::new(&e.to_string())
            )
        };
        let response: CreateTokenResponse = CreateTokenResponse{
            api_token: token.token
        };
        Ok(HttpResponse::Ok().json(response))
    }
    else {
        Err::<HttpResponse,KleahErr>(
            KleahErr::new("Password integrity could not be verified.")
        )
    }
}

#[post("/api/user/token/delete")]
pub async fn delete_api_token_service(
    payload: Json<UserChangePayload>,
    data: Data<AppData>
) -> Result<HttpResponse, KleahErr>{
    let user: KleahUser = match get_user_by_token(
        &payload.api_token,
        &data.pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<HttpResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let verified: bool = match verify(
        &payload.new_entity,
        &user.password
    ){
        Ok(verified) => verified,
        Err(e) => return Err::<HttpResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    if verified {
        let status: bool = match destroy_token(
            &user.username, 
            &payload.api_token, 
            &data.pool
        ).await {
            Ok(_f) => true,
            Err(_e) => false
        };
        let resp: StatusResponse = StatusResponse{
            status: status
        };
        Ok(HttpResponse::Ok().json(resp))
    }
    else {
        Err::<HttpResponse,KleahErr>(
            KleahErr::new("Password integrity could not be verified.")
        )
    }
}
