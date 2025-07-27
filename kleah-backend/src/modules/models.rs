/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "FromRow"
/// derive macro for reading
/// and writing data to and
/// from the database.
use sqlx::FromRow;

/// Importing the
/// macro for 
/// serializing Rust
/// data structures
/// as JSON.
use serde::Serialize;

/// A model for storing
/// information about 
/// the Kleah instance.
#[derive(FromRow)] // done.
pub struct InstanceInfo{
    pub instance_name: String,
    pub instance_host: String,
    pub instance_smtp: String,
    pub instance_pass: String,
    pub instance_admin: String,
    pub instance_description: String,
    pub default_primary: String,
    pub default_secondary: String,
    pub default_tertiary: String
}

/// A model for storing
/// a user's API tokens.
#[derive(FromRow, Serialize)] // done.
pub struct UserAPIToken{
    pub token: String,
    pub user_id: String
}

/// A model to store
/// information on a 
/// note a user has
/// created.
#[derive(FromRow, Serialize)]
pub struct Note{
    pub note_id: String,
    pub author: String,
    pub published: String,
    pub content: String,
    pub like_count: i32,
    pub boost_count: i32,
    pub share_count: i32,
    pub is_reply: bool,
    pub reply_to: String,
    pub sensitive: bool
}

/// A model for storing
/// confidential info
/// of an actor.
#[derive(FromRow)] // done.
pub struct PrivateActor{
    pub username: String,
    pub email: String,
    pub verified: bool,
    pub privileged: bool,
    pub private_key: String,
    pub public_key: String,
    pub user_password: String,
    pub default_primary: String,
    pub default_secondary: String,
    pub default_tertiary: String
}

/// A model for storing
/// info pertaining to
/// an actor.
#[derive(FromRow, Serialize)] // done.
pub struct Actor{
    pub user_id: String,
    pub host: String,
    pub user_type: String,
    pub preferred_username: String,
    pub display_name: String,
    pub summary: String,
    pub manually_approves_followers: bool,
    pub discoverable: bool,
    pub indexable: bool,
    pub published: String,
    pub memorial: bool,
}

/// A structure to model
/// an activity executed by
/// an actor in the database.
#[derive(FromRow)]
pub struct UserAct {
  pub activity_id: String,
  pub activity_type: String,
  pub activity_author: String,
  pub published_at: String,
  pub object_id: String
}

/// A model to represent 
/// a file a user has uploaded
/// in the database.
#[derive(FromRow)]
pub struct UserFile{
    pub file_id: String,
    pub file_owner: String,
    pub file_path: String,
    pub visibility: bool
}

//// A structure to model
/// a relationship of a 
/// like on a note.
#[derive(FromRow)]
pub struct UserLike{
    pub user_id: String,
    pub note_id: String
}
