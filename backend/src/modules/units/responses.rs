/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Serialize"
/// trait to derive it and 
/// serialize Rust structures
/// as JSON.
use serde::Serialize;

/// Importing the "Charm"
/// structure to work with charms
/// and explicitly declare
/// them.
use crate::models::Charm;

/// A data structure
/// that returns feedback on
/// whether an operation
/// was successful or not.
#[derive(Serialize)]
pub struct StatusResponse{
    pub status: usize
}

/// A data structure that
/// returns all relevant
/// information on a user.
#[derive(Serialize)]
pub struct UserProfile{
    pub user_role: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub banner_url: String,
    pub user_description: String,
    pub follower_count: usize,
    pub following_count: usize,
    pub charm_count: usize
}

/// A data structure that
/// returns all relevant
/// information on a user's
/// timeline.
#[derive(Serialize)]
pub struct UserTimeline {
    pub charms: Vec<CharmDetail>
}

/// A data structure that
/// returns all relevant
/// information on a user's
/// timeline.
#[derive(Serialize)]
pub struct CharmDetail {
    pub username: String,
    pub avatar_url: String,
    pub created_at: String,
    pub display_name: String,
    pub charm_text: String,
    pub file_url: Option<String>,
    pub like_count: i32,
    pub reaction_count: i32
}