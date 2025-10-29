/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the function
/// to read environment
/// variables.
use std::env::var;

use actix_web::App;

use actix_web::web::Data;

use super::units::AppData;

/// Importing the function to
/// check the validity of an
/// email address string to test
/// it.
use super::utils::check_email;

/// Importing the function to retrieve
/// user information from the database
/// to test it.
use super::db::get_user_by_id;

/// Importing the function to
/// create a new user to test it.
use super::db::create_new_user;

/// Importing the function to retrieve
/// actor information from the database
/// to test it.
use super::db::get_actor_by_id;

/// Importing the function to
/// create a new actor to test it.
use super::db::create_new_actor;

/// Importing the function to retrieve
/// instance information from the database
/// to test it.
use super::db::get_instance_info;

/// Importing the function to
/// check the validity of a
/// username string to test
/// it.
use super::utils::check_username;

/// Importing the function to
/// check the validity of a
/// password string to test
/// it.
use super::utils::check_password;

/// Importing the function to generate
/// an RSA keypari to test it.
use super::utils::generate_keypair;

use super::payloads::KleahUserType;

/// Importing the function to create
/// a connection to the database to
/// test it.
use super::utils::create_connection;

/// Importing the API service function
/// for creating a new user to test it.
use super::api::create_user_service;

/// Importing the function to
/// create a new instance information 
/// record to test it.
use super::db::create_instance_info;

use super::payloads::UserCreatePayload;

use actix_web::http::header::ContentType;

/// The function to test functions
/// from the module containing utlity
/// functions.
#[tokio::test]
pub async fn test_utility_functions(){
    let email_t: bool = check_email("example@example.com");
    let email_f: bool = check_email("example@example@com");
    let username_t: bool = check_username("alyxshang");
    let username_f: bool = check_username("alyxshang-1234");
    let password_t: bool = check_password("1234567890123456");
    let password_f: bool = check_password("12345678901234567");
    let gen_kp = generate_keypair();
    let db_url: String = var("KLEAH_DB_URL")
        .expect("Failed to read environment variable.");
    let connection = create_connection(&db_url).await;
    assert_eq!(email_t, true);
    assert_eq!(email_f, false);
    assert_eq!(username_t, true);
    assert_eq!(username_f, false);
    assert_eq!(password_t, true);
    assert_eq!(password_f, false);
    assert_eq!(gen_kp.is_ok(), true);
    assert_eq!(connection.is_ok(), true);
}

/// The function to test functions
/// from the module containing
/// functions to read from and write to
/// the database.
#[tokio::test]
pub async fn test_database_functions(){
    let db_url: String = var("KLEAH_DB_URL")
        .expect("Failed to read environment variable.");
    let connection = create_connection(&db_url).await
        .expect("Could not create connection.");
    let inst = create_instance_info("example.com", &connection).await
        .expect("Could not create instance information.");
    let inst_i = get_instance_info(&connection).await
        .expect("Could not fetch instance information.");
    let user = create_new_user(
        "Alyx Shang", 
        "123456789", 
        "alyxshang", 
        "me@example.com", 
        "A person of interest.", 
        &connection
    ).await
        .expect("Could not create user.");
    let actor = create_new_actor(
        "Alyx Shang", 
        "example.com", 
        "alyxshang", 
        "Person", 
        "A person of interest.", 
        "/apub/alyxshang/liked", 
        "/apub/alyxshang/inbox", 
        "/apub/alyxshang/outbox", 
        "/apub/alyxshang/following", 
        "/apub/alyxshang/followers", 
        "/apub/alyxshang/pubkey", 
        &connection
    ).await
        .expect("Could not create actor.");
    let created_u = get_user_by_id(
        "alyxshang", 
        &connection
    ).await
        .expect("Could not fetch user.");
    let created_a = get_actor_by_id(
        "alyxshang", 
        "example.com", 
        &connection
    ).await
        .expect("Could not fetch user.");
    assert_eq!(inst.host, "example.com");
    assert_eq!(inst_i.host, "example.com");
    assert_eq!(user.username, "alyxshang");
    assert_eq!(actor.username, "alyxshang");
    assert_eq!(created_u.username, "alyxshang");
    assert_eq!(created_a.username, "alyxshang");

}

/// The function to test functions
/// from the module containing service
/// functions for the REST API.
#[actix_web::test]
pub async fn test_api_services(){
    use actix_web::test;
    let db_url: String = var("KLEAH_DB_URL")
        .expect("Failed to read environment variable.");
    let connection = create_connection(&db_url).await
        .expect("Could not create connection.");
    let cu_tr_payload: UserCreatePayload = UserCreatePayload{
        name: "Robert Madison".to_string(),
        password: "123456789".to_string(),
        username: "robbymad".to_string(),
        email_addr: "me@example.com".to_string(),
        description: "Son of Mad Robert.".to_string(),
        user_type: KleahUserType::Person
    };
    let app = test::init_service(
        App::new()
            .app_data(Data::new(AppData{ pool: connection }))
            .service(create_user_service)
    ).await;
    let cu_tr = test::TestRequest::post()
        .uri("/api/user/create")
        .insert_header(ContentType::json())
        .set_json(cu_tr_payload)
        .to_request();
    let cu_tr_resp = test::call_service(&app, cu_tr).await;
    assert_eq!(cu_tr_resp.status().is_success(), true);
    
}
