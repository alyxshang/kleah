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

/// Importing the "App"
/// structure to create a new
/// Actix Web app.
use actix_web::App;

/// Importing this crate's
/// error structure.
use crate::KleahErr;

/// Importing the "Cors"
/// structure to add CORS
/// rules.
use actix_cors::Cors;

/// Importing the "get"
/// function to register a 
/// "GET" service.
use actix_web::web::get;

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

/// Importing the "AppData"
/// structure to register
/// persistent app data.
use crate::config::AppData;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// Importing the "ConfigData"
/// structure to capture and use
/// configuration data.
use crate::config::ConfigData;

/// Importing the "create_connection"
/// function to create a connection
/// to the PostgreSQL database.
use crate::utils::create_connection;

/// Importing the "DefaultHeaders" structure
/// to set custom headers.
use actix_web::middleware::DefaultHeaders;

/// Attempts to run the app with the supplied instance of the
/// "ConfigData" structure.s
pub async fn run_app(config: &ConfigData) -> Result<(), KleahErr> {
    let app_addr: String = format!("{}:{}", config.actix_host, config.actix_port);
    let connection: Pool<Postgres> = match create_connection(&config.db_url).await{
        Ok(connection) => connection,
        Err(e) => return Err::<(), KleahErr>(KleahErr::new(&e.to_string()))
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
        }
    ).bind(app_addr){
        Ok(server) => server,
        Err(e) => return Err::<(), KleahErr>(KleahErr::new(&e.to_string()))
    };
    let running: () = match server.run().await{
        Ok(running) => running,
        Err(e) => return Err::<(), KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(running)
}