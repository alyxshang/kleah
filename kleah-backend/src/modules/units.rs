/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// representing a pool of
/// database connections.
use sqlx::Pool;

/// Importing the structure that
/// represents a connection to a 
/// PostgreSQL database.
use sqlx::postgres::Postgres;

/// A data structure to
/// "store" a pool of
/// connections to a 
/// PostgreSQL database
/// while an app is running.
pub struct AppData{
    pub pool: Pool<Postgres>
}

/// A structure to hold
/// information on public
/// and private keys, 
/// respectively.
pub struct KeyPair {
    pub private_key: String,
    pub public_key: String
}
