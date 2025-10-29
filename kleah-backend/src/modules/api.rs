/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

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

/// Importing the "Httpresponse"
/// structure to return errors
/// as HTTP responses.
use actix_web::HttpResponse;

/// Importing the data structure
/// modelling data about a Kleah user
/// in the database.
use super::models::KleahUser;

/// Importing the data structure
/// modelling data about an ActivityPub
/// actor on a Kleah instance in the 
/// database.
use super::models::KleahActor;

/// Importing the function to check
/// whether a supplied string is a
/// valid email address.
use super::utils::check_email;

/// Importing the function to
/// create a record for a new
/// Kleah user in the database.
use super::db::create_new_user;

/// Importing the function to
/// create a record for a new
/// Kleah ActivityPub actor
/// in the database.
use super::db::create_new_actor;

/// Importing the function for
/// retrieving information about
/// the current Kleah instance.
use super::db::get_instance_info;

/// Importing the function to check
/// whether a supplied string is a
/// valid username.
use super::utils::check_username;

/// Importing the function to check
/// whether a supplied string is a
/// valid password.
use super::utils::check_password;

/// Importing the enumeration describing
/// the types of Kleah users that can
/// exist.
use super::payloads::KleahUserType;

/// Importing the data structure
/// modelling data about the current
/// Kleah instance in the database.
use super::models::InstanceInformation;

/// Importing the structure representing
/// a JSON payload containing data for 
/// creating a new Kleah user.
use super::payloads::UserCreatePayload;

/// Importing the structure for serializing 
/// a Rust data structure containing data
/// on a created Kleah user and actor into
/// a JSON string.
use super::responses::UserCreateResponse;

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
    if check_username(&payload.username) &&
       check_password(&payload.password) &&
       check_email(&payload.email_addr)
    {
        let kleah_user: KleahUser = match create_new_user(
            &payload.name,
            &payload.password,
            &payload.username,
            &payload.email_addr,
            &payload.description,
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
