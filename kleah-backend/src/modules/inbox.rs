/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Pool"
/// structure to use a pool
/// of connections.
use sqlx::Pool;

/// Importing the "get"
/// decorator to make a service
/// that accepts "GET" requests.
use actix_web::get;

/// Importing the "post"
/// decorator to make a service
/// that accepts "POST" requests.
use actix_web::post;

/// Importing the "Result"
/// enum for Actix Web services.
use actix_web::Result;

/// Importing the "Data"
/// structure to register
/// persistent app data.
use actix_web::web::Data;

/// Importing this crate's
/// error structure.
use super::err::KleahErr;

/// Importing the "HttpRequest"
/// structure to check properties
/// of incoming requests.
use actix_web::HttpRequest;

/// Importing the "Json"
/// structure to return JSON
/// responses.
use actix_web::web::Json;

/// Importing the data structure
/// for creating a public actor.
use super::models::Actor;

/// importing the "Path"
/// entity to capture
/// URL parameters.
use actix_web::web::Path;

/// Importing the data structure
/// to have one SQL pool for
/// allservice functions.
use super::units::AppData;

/// Importing the function
/// to return a HTTP response.
use actix_web::HttpResponse;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// Importing the data structure
/// for structred inbox sending
/// and receiving operations.
use super::units::ActivityNotify;

/// Importing the function to create
/// a new user in the database.
use super::database::create_user;

/// Importing the structure
/// to capture and deserialize
/// URL parameters.
use super::units::WebFingerResource;

/// A service function to provide
/// an API endpoint for posting activities
/// to a user's inbox. If the operation
/// is successful, an HTTP response with
///  a success code is sent. If it fails,
///  an error is returned.
#[post("/{username}/inbox")]
pub async fn user_inbox(
    req: HttpRequest,
    path: Path<String>
) -> Result<HttpResponse, KleahErr>{
    let user: String = path.into_inner();
    Ok(HttpResponse::Accepted().finish())
}

/// A function to post activities
/// to a foreign actor's inbox,
/// if a user on the local instance
/// has interacted with a user on
/// a remote instance. If the operation
/// is successful nothing is returned.
/// If the operation fails, an error is
/// returned.
pub async fn post_to_inbox(
    payload: ActivityNotify,
    pool: &Pool<Postgres>
) -> Result<(), KleahErr>{
    Ok(())
}
