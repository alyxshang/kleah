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

/// Importing the
/// macro for 
/// serializing Rust
/// data structures
/// as JSON.
use serde::Serialize;

/// Importing the "Deserialize"
/// macro to use for JSON.
use serde::Deserialize;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// A structure representing
/// a JSON payload to signup
/// a user to the platform.
#[derive(Deserialize)]
pub struct SignUpUserPayload{
    pub username: String,
    pub password: String,
    pub display_name: String,
    pub email_addr: String,
    pub user_type: String
}

/// A structure to capture
/// URL parameters from an
/// Actix Web path.
#[derive(Deserialize)]
pub struct WebFingerResource{
    pub username: String,
    pub domain: String,
}

/// A structure containing
/// a pool of database connections
/// to make app data persist.
pub struct AppData {
    pub pool: Pool<Postgres>
}

/// Implementing generic
/// methods for the "AppData"
/// structure.
impl AppData{

    /// Implementing a method
    /// to create a new instance
    /// of the "AppData"
    /// structure.
    pub fn new(pg_pool: &Pool<Postgres>) -> AppData{
        AppData { pool: pg_pool.to_owned() }
    }

}

/// A structure to encapsulate
/// key pair data as strings.
pub struct KeyPair{
    pub private: String,
    pub public: String
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum ActivityNotifySubObject {
    Uri(String),
    Embedded(ActivityNotifyObject),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ActivityNotify {
    #[serde(rename(deserialize = "@context", serialize = "@context"))]
    pub context: String,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub activity_type: String,
    pub actor: String,
    pub object: ActivityNotifySubObject
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ActivityNotifyObject {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub action_type: String,
    pub actor: String,
    pub object: String
}
