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
