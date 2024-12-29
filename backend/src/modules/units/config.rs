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

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// A structure containing
/// a pool of database connections
/// to make app data persist.
pub struct AppData {
    pub pool: Pool<Postgres>,
    pub smtp_server: String,
    pub instance_url: String,
    pub file_folder: String
}

/// Implementing generic
/// methods for the "AppData"
/// structure.
impl AppData{

    /// Implementing a method
    /// to create a new instance
    /// of the "AppData"
    /// structure.
    pub fn new(
        pg_pool: &Pool<Postgres>, 
        smtp_server: &String,
        instance_url: &String,
        file_folder: &String,
    ) -> AppData{
        AppData { 
            pool: pg_pool.to_owned(), 
            smtp_server: smtp_server.to_owned(),
            instance_url: instance_url.to_owned(),
            file_folder: file_folder.to_owned()
        }
    }

}

/// A structure containing
/// the fields required to run the
/// backend.
pub struct ConfigData{
    pub db_url: String,
    pub actix_host: String,
    pub actix_port: String,
    pub smtp_server: String,
    pub instance_url: String,
    pub file_folder: String,
}

/// Implementing generic
/// methods for the "ConfigData"
/// structure.
impl ConfigData{

    /// Implementing a method
    /// to create a new instance
    /// of the "ConfigData"
    /// structure.
    pub fn new(
        db_url: &String,
        actix_host: &String,
        actix_port: &String,
        smtp_server: &String,
        instance_url: &String,
        file_folder: &String
    ) -> ConfigData {
        ConfigData {
            db_url: db_url.to_owned(),
            actix_host: actix_host.to_owned(),
            actix_port: actix_port.to_owned(),
            smtp_server: smtp_server.to_owned(),
            instance_url: instance_url.to_owned(),
            file_folder: file_folder.to_owned()
        }
    }
    
}