/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Deserialize"
/// macro for deserializing Rust
/// data structures from JSON.
use serde::Deserialize;

#[derive(Deserialize)]
pub struct InviteCreationPayload{
    pub api_token: String,
    pub code: String
}

#[derive(Deserialize)]
pub struct InviteDeletionPayload{
    pub code_id: String
}
