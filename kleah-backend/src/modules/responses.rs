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

/// A structure to contain data
/// about a created user and serialize
/// this data into a JSON string.
#[derive(Serialize)]
pub struct UserCreateResponse{
    pub name: String,
    pub username: String,
    pub description: String
}

/// A structure to contain data
/// about a created user API token 
/// and serialize this data into 
/// a JSON string.
#[derive(Serialize)]
pub struct CreateTokenResponse{
    pub api_token: String
}

/// A structure to contain data
/// about whether a change to
/// a user's record(s) in the 
/// database was successful 
/// or not and serialize 
/// this data into a JSON 
/// string.
#[derive(Serialize)]
pub struct StatusResponse{
    pub status: bool
}
