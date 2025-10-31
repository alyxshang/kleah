/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// A macro from the SQLx
/// library to serialize 
/// and deserialize database
/// tables into Rust data
/// structures.
use sqlx::FromRow;

/// A structure to model
/// a Kleah user on a
/// Kleah instance in the
/// database.
#[derive(FromRow)]
pub struct KleahUser{
    pub name: String,
    pub username: String,
    pub password: String,
    pub email_addr: String,
    pub public_key: String,
    pub description: String,
    pub private_key: String,
    pub is_admin: bool
}

/// A structure to model
/// an ActivityPub actor on a
/// Kleah instance in the
/// database.
#[derive(FromRow)]
pub struct KleahActor{
    pub name: String,
    pub actor_type: String,
    pub host: String,
    pub liked: String,
    pub inbox: String,
    pub outbox: String,
    pub following: String,
    pub followers: String,
    pub username: String,
    pub description: String,
    pub public_key: String,
}

/// A structure to model
/// information about the 
/// current Kleah instance
/// in the database.
#[derive(FromRow)]
pub struct InstanceInformation{
    pub host: String,
    pub uses_invites: bool
}

/// A structure to model
/// information about a
/// user's API token in the
/// database.
#[derive(FromRow)]
pub struct UserAPIToken{
    pub username: String,
    pub token: String
}

#[derive(FromRow)]
pub struct InviteCode{
    pub code: String,
}

#[derive(FromRow)]
pub struct UserFile{
    pub file_id: String,
    pub owner: String,
    pub is_public: bool,
    pub file_path: String
}
