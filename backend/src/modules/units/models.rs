/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

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

/// A data structure containing information
/// on a Kleah user.
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct KleahUser {
    pub user_id: String,
    pub user_role: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub banner_url: String,
    pub user_description: String,
    pub email: String,
    pub pwd: String,
    pub email_token: String,
    pub is_active: bool,
    pub rules_accepted: bool,
    pub is_admin: bool
}

/// A data structure containing information
/// on a post a Kleah user makes.
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct Charm {
    pub user_id: String,
    pub charm_id: String,
    pub charm_text: String,
    pub created_at: String,
    pub file_id: Option<String>,
    pub is_reply: bool,
    pub refers_to: Option<String>,
    pub reaction_ids: Option<String>,
    pub like_count: Option<i32>,
    pub reaction_count: Option<i32>,
}

/// A data structure a file
/// a Kleah user has uploaded.
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct KleahUserFile {
    pub file_id: String,
    pub user_id: String,
    pub file_name: String,
    pub file_path: String
}


/// This structure models the relationship
/// of one Kleah user following another.
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct KleahUserFollows {
    pub relationship_id: String,
    pub follower: String,
    pub followee: String,
}

/// This structure models the relationship
/// of one Kleah user following another.
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct KleahUserBlocks {
    pub block_id: String,
    pub blocker: String,
    pub blockee: String,
}

/// This structure models the information
/// about an instance set by an admin user.
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct InstanceInfo {
    pub instance_id: String,
    pub instance_description: String,
    pub instance_name: String,
    pub kleah_version: String,
    pub admin_user_id: String,
    pub instance_rules: String
}

/// This structure models an API token
/// for a Kleah user.
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct APIToken {
    pub user_id: String,
    pub token: String,
    pub created_at: String,
    pub is_active: bool,
    pub can_change_pwd: bool,
    pub can_change_username: bool,
    pub can_post_charms: bool,
    pub can_delete_user: bool,
    pub can_change_email: bool
}

/// This structure models an invite code
/// issued by an admin user.
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct InviteCode {
    pub user_id: String,
    pub invite_code: String
}

/// This structure models a like reaction
/// issued by a user on a Kleah charm.
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct UserLike {
    pub like_id: String,
    pub user_id: String,
    pub charm_id: String
}

/// A structure for modelling a
/// user-created theme. This theme is
/// saved and owned by the user.
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct UserTheme {
    pub theme_id: String,
    pub theme_owner: String,
    pub theme_name: String,
    pub primary_color: String,
    pub accent_color: String
}