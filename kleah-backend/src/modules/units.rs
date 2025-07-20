/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// "Pool" structure
/// from the "sqlx" crate
/// to make a pool for
/// database connections.
use sqlx::Pool;

/// Importing the
/// macro for 
/// serializing Rust
/// data structures
/// as JSON.
use serde::Serialize;

use serde_json::Value;

/// Importing the "Deserialize"
/// macro to use for JSON.
use serde::Deserialize;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

use std::collections::HashMap;

/// A structure representing
/// a JSON payload to signup
/// a user to the platform.
#[derive(Deserialize)]
pub struct SignUpUserPayload{
    pub username: String,
    pub password: String,
    pub display_name: String,
    pub email_addr: String,
    pub user_type: String
}

/// A structure to capture
/// URL parameters from an
/// Actix Web path.
#[derive(Deserialize)]
pub struct WebFingerResource{
    pub username: String,
    pub domain: String,
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

/// A structure to encapsulate
/// key pair data as strings.
pub struct KeyPair{
    pub private: String,
    pub public: String
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum ActivityNotifySubObject {
    Uri(String),
    Embedded(ActivityNotifyObject),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ActivityNotify {
    #[serde(rename(deserialize = "@context", serialize = "@context"))]
    pub context: String,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub activity_type: String,
    pub actor: String,
    pub object: ActivityNotifySubObject
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ActivityNotifyObject {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub action_type: String,
    pub actor: String,
    pub object: String
}

#[derive(Deserialize, PartialEq, Debug, Serialize)]
pub struct WebFingerLink {
    pub rel: String,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub content_type: Option<String>,
    pub href: Option<String>,
    pub template: Option<String>
}

#[derive(Deserialize, PartialEq, Debug, Serialize)]
pub struct WebFingerResponse {
    pub subject: String,
    pub aliases: Vec<String>,
    pub links: Vec<WebFingerLink>
}

#[derive(Deserialize, PartialEq, Debug, Serialize)]
pub struct ActorWebFingerInfo{
    #[serde(rename(deserialize = "@context"))]
    pub context: Option<Value>,
    pub id: String,
    #[serde(rename(deserialize = "type"))]
    pub entity_type: String,
    pub following: String,
    pub followers: String,
    pub inbox: String,
    pub outbox: String,
    pub featured: String,
    #[serde(rename(deserialize = "featuredTags"))]
    pub featured_tags: String,
    #[serde(rename(deserialize = "preferredUsername"))]
    pub preferred_username: String,
    pub name: String,
    pub summary: String,
    pub url: String,
    #[serde(rename(deserialize = "manuallyApprovesFollowers"))]
    pub manually_approves_followers: bool,
    pub discoverable: bool,
    pub indexable: bool,
    pub published: String,
    pub memorial: bool,
    #[serde(rename(deserialize = "publicKey"))]
    pub public_key: ActorPublicKey,
    pub tag: Vec<String>,
    pub attachment: Option<Vec<ActorAttachment>>,
    pub endpoints: HashMap<String, String>,
    pub icon: ActorImage,
    pub image: ActorImage
}

#[derive(Deserialize, PartialEq, Debug, Serialize)]
pub struct ActorImage {
    #[serde(rename(deserialize = "type"))]
    pub image_type: String,
    #[serde(rename(deserialize = "mediaType"))]
    pub media_type: String,
    pub url: String,
}

#[derive(Deserialize, PartialEq, Debug, Serialize)]
pub struct ActorPublicKey {
    pub id: String,
    pub owner: String,
    #[serde(rename(deserialize = "publicKeyPem"))]
    pub public_key_pem: String
}

#[derive(Deserialize, PartialEq, Debug, Serialize)]
pub struct ActorAttachment {
    #[serde(rename(deserialize = "type"))]
    pub prop_type: String,
    pub name: String,
    pub value: String
}

#[derive(Deserialize, PartialEq, Debug, Serialize)]
pub struct ActorOutBoxInfo {
    #[serde(rename(deserialize = "@context"))]
    pub context: String,
    pub id: String,
    #[serde(rename(deserialize = "type"))]
    pub coll_type: String,
    #[serde(rename(deserialize = "totalItems"))]
    pub total_items: String,
    pub first: String,
    pub last: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Activity {
    pub id: String,
    #[serde(rename(deserialize = "type"))]
    pub act_type: String,
    pub actor: String,
    pub published: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub object: DetailedActivity
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct DetailedActivity{
    pub id: String,
    #[serde(rename(deserialize = "type"))]
    pub entity_type: String,
    pub summary: Option<String>,
    #[serde(rename(deserialize = "inReplyTo"))]
    pub in_reply_to: Option<String>,
    pub published: String,
    pub url: String,
    #[serde(rename(deserialize = "attributedTo"))]
    pub attributed_to: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub sensitive: bool,
    pub atom_uri: String,
    #[serde(rename(deserialize = "inReplyToAtomUri"))]
    pub in_reply_to_atom_uri: Option<String>,
    pub conversation: String,
    pub content: String,
    pub content_map: String,
    pub updated: String,
    pub attachment: Option<Vec<ActorAttachment>>,
    pub tag: Vec<ActorHashTag>,
    pub replies: Option<ReplyItem>,
    pub likes: AttribObject
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ActorHashTag {
    #[serde(rename(deserialize = "type"))]
    pub entity_type: String,
    pub href: String,
    pub name: String
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ReplyItem {
    pub id: String,
    #[serde(rename(deserialize = "type"))]
    pub coll_type: String,
    pub first: IndexEntity,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct IndexEntity {
    #[serde(rename(deserialize = "type"))]
    pub coll_type: String,
    pub next: String,
    pub part_of: String,
    pub items: Vec<Activity>
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct AttribObject {
    pub id: String,
    #[serde(rename(deserialize = "type"))]
    pub coll_type: String,
     #[serde(rename(deserialize = "totalItems"))]
    pub total_items: usize,
}

#[derive(Deserialize)]
pub struct LoginUserPayload{
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct LogoutUserPayload{
    pub token: String
}
