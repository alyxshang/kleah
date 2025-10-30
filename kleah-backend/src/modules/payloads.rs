/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the macro
/// to serialize Rust
/// data structures
/// into a JSON
/// string.
use serde::Serialize;

/// Importing the macro
/// to deserialize JSON 
/// strings into Rust 
/// data structures.
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub enum KleahUserType{
    Bot,
    Person
}

/// Declaring a data structure
/// that models data in a JSON 
/// string received for creating
/// a new Kleah user.
#[derive(Serialize, Deserialize)]
pub struct UserCreatePayload{
    pub name: String,
    pub password: String,
    pub username: String,
    pub email_addr: String,
    pub description: String,
    pub user_type: KleahUserType
}

/// Declaring a data structure
/// that models data in a JSON 
/// string received for making
/// a trivial change to the
/// record of a Kleah user in
/// the database.
#[derive(Serialize, Deserialize)]
pub struct UserChangePayload{
    pub api_token: String,
    pub new_entity: String
}

/// Declaring a data structure
/// that models data in a JSON 
/// string received for making
/// a significant change to the
/// record of a Kleah user in
/// the database.
#[derive(Serialize, Deserialize)]
pub struct SecureUserChangePayload{
    pub api_token: String,
    pub new_entity: String,
    pub old_entity: String,
}

/// Declaring a data structure
/// that models data in a JSON 
/// string received for creating
/// a new API token for a Kleah user.
#[derive(Serialize, Deserialize)]
pub struct CreateTokenPayload{
    pub username: String,
    pub password: String
}
