/*
Jade by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// "Pool" structure
/// from the "sqlx" crate
/// to make a pool for
/// database connections.
use sqlx::Pool;

/// Importing the "App"
/// structure to create a new
/// Actix Web app.
use actix_web::App;

/// Importing the "Cors"
/// structure to add CORS
/// rules.
use actix_cors::Cors;

/// Importing the "get"
/// function to register a 
/// "GET" service.
use actix_web::web::get;

/// Importing the service
/// function for setting
/// a user's current mood.
use super::api::set_mood;

/// Importing this crate's
/// error structure.
use super::err::JadeErr;

/// Importing the service
/// function for getting
/// a user's current mood.
use super::api::get_mood;

/// Importing the "post"
/// function to register a 
/// "POST" service.
use actix_web::web::post;

/// Importing the "Data"
/// structure to register
/// persistent app data.
use actix_web::web::Data;

/// Importing the "HttpServer"
/// structure to create an
/// Actix Web app.
use actix_web::HttpServer;

/// Importing the service
/// function for getting
/// a user's moods.
use super::api::get_moods;

/// Importing the "AppData"
/// structure to register
/// persistent app data.
use super::units::AppData;

/// Importing the service
/// function for getting
/// a user's API tokens.
use super::api::get_tokens;

/// Importing the service
/// function for deleting
/// a user's mood.
use super::api::delete_mood;

/// Importing the service
/// function for deleting
/// a user.
use super::api::delete_user;

/// Importing the service
/// function for creating
/// a user.
use super::api::create_user;

/// Importing the service
/// function for verifying
/// a user's email.
use super::api::verify_email;

/// Importing the "ConfigData"
/// structure for explicit typing.
use super::units::ConfigData;

/// Importing the service
/// function for creating
/// an API token for a user.
use super::api::create_token;

/// Importing the service
/// function for deleting
/// a user's API token.
use super::api::delete_token;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// Importing the service
/// function for changing
/// a user's password.
use super::api::change_user_pwd;

/// Importing the service
/// function for uploading
/// a file.
use super::api::upload_user_file;

/// Importing the service
/// function for changing
/// a user's email address.
use super::api::change_user_email;

/// Importing the "create_connection"
/// function to create a connection
/// to the PostgreSQL database.
use super::utils::create_connection;

/// Importing the "DefaultHeaders" structure
/// to set custom headers.
use actix_web::middleware::DefaultHeaders;

/// Attempts to run the app with the supplied instance of the
/// "ConfigData" structure.s
pub async fn run_app(config: &ConfigData) -> Result<(), JadeErr> {
    let app_addr: String = format!("{}:{}", config.actix_host, config.actix_port);
    let connection: Pool<Postgres> = match create_connection(&config.db_url).await{
        Ok(connection) => connection,
        Err(e) => return Err::<(), JadeErr>(JadeErr::new(&e.to_string()))
    };
    let data: Data<AppData> = Data::new(AppData::new(&connection, &config.smtp_server, &config.instance_url, &config.file_folder));
    let server = match HttpServer::new(
        move || {
            let cors = Cors::permissive()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST"]);
            App::new()
                .wrap(cors)
                .wrap(DefaultHeaders::new()
                    .add(("Access-Control-Allow-Origin", "*"))
                    .add(("Access-Control-Allow-Methods", "GET,POST"))
                    .add(("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept"))
                )
                .app_data(data.clone())
                .route("/token/create", post().to(create_token))
                .route("/token/delete", post().to(delete_token))
                .route("/user/delete", post().to(delete_user))
                .route("/user/create", post().to(create_user))
                .route("/mood/create", post().to(set_mood))
                .route("/mood/delete", post().to(delete_mood))
                .route("/user/update/pwd", post().to(change_user_pwd))
                .route("/user/update/email", post().to(change_user_email))
                .route("/mood/get", get().to(get_mood))
                .route("/moods/get", get().to(get_moods))
                .route("/tokens/get", get().to(get_tokens))
                .service(upload_user_file)
                .service(verify_email)
        }
    ).bind(app_addr){
        Ok(server) => server,
        Err(e) => return Err::<(), JadeErr>(JadeErr::new(&e.to_string()))
    };
    let running: () = match server.run().await{
        Ok(running) => running,
        Err(e) => return Err::<(), JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(running)
}