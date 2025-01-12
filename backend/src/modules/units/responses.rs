/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Value"
/// entity to map unserializable
/// values.
use serde_json::Value;

/// Importing the "Serialize"
/// trait to derive it and 
/// serialize Rust structures
/// as JSON.
use serde::Serialize;

/// Importing the "Serialize"
/// trait to derive it and 
/// serialize Rust structures
/// as JSON.
use serde::Deserialize;

/// Importing the standard
/// Rust structure for maps.
use std::collections::HashMap;

/// Importing the structure
/// that models a user-created file.
use crate::models::KleahUserFile;

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

pub struct UserFilesTimeline {
    pub files: Vec<KleahUserFile>
}

#[derive(Serialize)]
pub struct ActorFollows {
  #[serde(rename(serialize = "type"))]
  pub list_type: String,
  pub total_items: usize,
  pub items: Vec<String>
}

#[derive(Deserialize, Serialize)]
pub struct WebFingerInfo {
   pub subject: String,
   pub aliases: Vec<String>,
   pub links: Vec<WebFingerLink>
}

#[derive(Deserialize, Serialize)]
pub struct WebFingerLink{
   pub rel: String,
   #[serde(rename(serialize = "type", deserialize = "type"))]
   pub link_type: String,
   pub href: String,
}

#[derive(Deserialize, Serialize)]
pub struct SubscriptionLink{
   pub rel: String,
   pub template: String
}

#[derive(Deserialize, Serialize)]
pub enum UserType {
  Person,
  Other
}

#[derive(Deserialize, Serialize)]
pub struct Actor {
  #[serde(rename(serialize = "@context"))]
  pub context: Vec<Value>,
  pub id: String,
  #[serde(rename(serialize = "type", deserialize = "type"))]
  pub user_type: UserType,
  pub following: String,
  pub followers: String,
  pub inbox: String,
  pub outbox: String,
  pub featured: String,
  #[serde(rename(serialize = "featuredTags"))]
  pub featured_tags: String,
  #[serde(rename(serialize = "preferredUsername"))]
  pub preferred_username: String,
  pub name: String,
  pub summary: String,
  pub url: String,
  #[serde(rename(serialize = "manuallyApprovesFollowers"))]
  pub manually_approves_followers: bool,
  pub discoverable: bool,
  pub indexable: bool,
  pub published: String,
  pub memorial: bool,
  pub public_key: PublicUserKey,
  pub tag: Vec<String>,
  pub attachment: Vec<Attachment>,
  pub endpoints: HashMap<String, String>,
  pub icon: ActorImage,
  pub image: ActorImage
}

#[derive(Deserialize, Serialize)]
pub struct Attachment{
  #[serde(rename(serialize = "type"))]
  pub att_type: String,
  pub name: String,
  pub value: String  
}

#[derive(Deserialize, Serialize)]
pub enum MediaType{
  PngImage,
  JpegImage
}

#[derive(Deserialize, Serialize)]
pub struct ActorImage{
  #[serde(rename(serialize = "type"))]
  pub im_type: String,
  #[serde(rename(serialize = "mediaType"))]
  pub media_type: MediaType,
  pub url: String
}

#[derive(Deserialize, Serialize)]
pub struct PublicUserKey{
  pub id: String,
  pub owner: String,
  #[serde(rename(serialize = "publicKeyPem"))]
  pub public_key_pem: String
}

#[derive(Deserialize, Serialize)]
pub struct ActorBox {
  #[serde(rename(serialize = "@context"))]
  pub context: String,
  pub id: String,
  #[serde(rename(serialize = "type"))]
  pub coll_type: String,
  #[serde(rename(serialize = "totalItems"))]
  pub total_items: usize,
  pub first: String,
  pub last: String
}

#[derive(Deserialize, Serialize)]
pub struct ActivityStreamPage {
  #[serde(rename(serialize = "@context"))]
  pub context: Vec<Value>,
  pub id: String,
  #[serde(rename(serialize = "@context"))]
  pub coll_type: String,
  pub next: Option<String>,
  pub prev: Option<String>,
  pub partOf: String,
  #[serde(rename(serialize = "orderedItems"))]
  pub activties: Vec<Activity>
}

#[derive(Deserialize, Serialize)]
pub enum ActivityType{
  Like,
  Create,
  Delete,
  Follow
}

#[derive(Deserialize, Serialize)]
pub enum PostType{
  Note,
  Create,
  Delete,
  Follow
}

#[derive(Deserialize, Serialize)]
pub struct Activity {
  pub id: String,
  #[serde(rename(serialize = "type"))]
  pub act_type: ActivityType,
  pub actor: String,
  pub published: String,
  pub to: Vec<String>,
  pub cc: Vec<String>,
  pub object: ActivityOboject
}

#[derive(Deserialize, Serialize)]
pub enum TagType {
  Mention
}

#[derive(Deserialize, Serialize)]
pub struct ActivityOboject{
  pub id: String,
  pub post_type: PostType,
  pub summary: Option<String>,
  #[serde(rename(serialize = "inReplyTo"))]
  pub in_reply_to: String,
  pub published: String,
  pub url: String,
  #[serde(rename(serialize = "attributedTo"))]
  pub attributed_to: String,
  pub to: Vec<String>,
  pub cc: Vec<String>,
  pub sensitive: bool,
  #[serde(rename(serialize = "atomUri"))]
  pub atom_uri: String,
  #[serde(rename(serialize = "inReplyToAtomUri"))]
  pub in_reply_to_atom_uri: String,
  pub conversation: String,
  pub content: String,
  pub content_map: HashMap<String,String>,
  pub attachment: Vec<String>,
  pub tag: Vec<ActivityTag>,
  pub replies: Option<String>,
  pub likes: ActivityInteraction,
  pub shares: ActivityInteraction

}

#[derive(Deserialize, Serialize)]
pub struct ActivityTag{
  #[serde(rename(serialize = "type"))]
  pub tag_type: TagType,
  pub href: String,
  pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct ActivityInteraction{
  pub id: String,
  #[serde(rename(serialize = "type"))]
  pub col_type: String,
  #[serde(rename(serialize = "totalItems"))]
  pub total_items: usize
}

#[derive(Deserialize, Serialize)]
pub struct ExtendedActivityInteraction{
  #[serde(rename(serialize = "@context"))]
  pub context: String,
  pub id: String,
  #[serde(rename(serialize = "type"))]
  pub col_type: String,
  #[serde(rename(serialize = "totalItems"))]
  pub total_items: usize
}

#[derive(Deserialize, Serialize)]
pub struct ActivityReply{
  pub id: String,
  pub col_type: String,
  pub first: FirstReplyEntry
}

#[derive(Deserialize, Serialize)]
pub enum EntryType {
  CollectionPage
}

#[derive(Deserialize, Serialize)]
pub  struct FirstReplyEntry{
  pub entry_type: EntryType,
  pub next: String,
  pub part_of: String,
  pub items: Vec<String>
}

#[derive(Deserialize, Serialize)]
pub struct FeaturedItems {
  pub context: Vec<Value>,
  pub id: String,
  pub col_type: String,
  #[serde(rename(serialize = "totalItems"))]
  pub total_items: usize,
  #[serde(rename(serialize = "orderedItems"))]
  pub ordered_items: Vec<Activity>
}