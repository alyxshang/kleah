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
