/*
Kleah Backend by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// "Pool" structure
/// from the "sqlx" crate
/// to make a pool for
/// database connections.
use sqlx::Pool;

/// Importing the 
/// "FromRow" trait
/// to derive it.
use sqlx::FromRow;

/// Importing the 
/// "Serialize" trait
/// to derive it.
use serde::Serialize;

/// Importing the 
/// "Deserialize" trait
/// to derive it.
use serde::Deserialize;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

#[derive(Serialize, Deserialize, FromRow)]
pub struct KleahUser {
    pub id: String,
    pub bio: String,
    pub pfp: DriveFile,
    pub banner: DriveFile,
    pub password: String,
    pub email: String,
    pub name: String,
    pub username: String,
    pub social_links: Vec<SocialLink>,
    pub charms: Vec<UserCharm>,
    pub followers: Box<KleahUser>,
    pub following: Box<KleahUser>,
    pub is_private: bool,
    pub users_blocked: Box<Vec<KleahUser>>,
    pub scrolls: Vec<UserScrolls>,
    pub instance_users: Box<Vec<KleahUser>>,
    pub users_to_accpet: Box<Vec<KleahUser>>,
    pub users_banned: Box<Vec<KleahUser>>,
    pub server_emojis: Vec<DriveFile>,
    pub invite_codes: Vec<InviteCode>,
    pub is_admin: bool
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserCharm {
    pub id: String,
    pub text: String,
    pub attachments: Option<Vec<DriveFile>>,
    pub reactions: Option<Vec<Reaction>>
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserScrolls {
    pub from_user: KleahUser,
    pub to_user: KleahUser,
    pub text: String,
    pub attachments: Option<Vec<DriveFile>>,
    pub reactions: Option<Reaction>
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct InviteCode {
    pub code: String
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct DriveFile {
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Reaction {

}

#[derive(Serialize, Deserialize, FromRow)]
pub struct SocialLink {

}

/// A structure containing
/// the fields required to run the
/// backend.
pub struct ConfigData{
    pub db_url: String,
    pub actix_host: String,
    pub actix_port: String
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
        actix_port: &String
    ) -> ConfigData {
        ConfigData {
            db_url: db_url.to_owned(),
            actix_host: actix_host.to_owned(),
            actix_port: actix_port.to_owned()
        }
    }
    
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