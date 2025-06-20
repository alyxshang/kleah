/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Serialize"
/// macro for serializing Rust
/// data structures into JSON.
use serde::Serialize;

/// Importing the "Deserialize"
/// macro for deserializing Rust
/// data structures from JSON.
use serde::Deserialize;

/// Importing the enum for
/// specifying the type of
/// actor.
use super::enums::UserType;

/// Importing the standard
/// library implementation
/// of a map.
use std::collections::HashMap;

/// Importing the enum for
/// specifying the type of
/// activity.
use super::enums::ActivityType;

/// Importing the enum for
/// specifying the type of
/// collection.
use super::enums::CollectionType;

/// Importing the enum for
/// specifying the type of
/// media submitted by an actor.
use super::enums::ActorMediaType;

/// Importing the enum for
/// specifying the type of
/// entity in an activity.
use super::enums::ActivityEntityType;

/// Importing the enum for
/// specifying the type of
/// attachment for an actor.
use super::enums::ActorAttachmentType;

/// A structure for returning
/// and receiving info about
/// media belonging to an actor.
#[derive(Serialize, Deserialize)]
pub struct ActorMedia{
    #[serde(rename(serialize = "type", deserialize="type"))]
    pub media_type: ActorMediaType,
    #[serde(rename(serialize = "mediaType", deserialize="mediaType"))]
    pub file_tag: String,
    pub url: String
}

/// A structure for returning
/// and receiving info about
/// an actor's public key.
#[derive(Serialize, Deserialize)]
pub struct ActorPublicKey{
    pub id: String,
    pub owner: String,
    #[serde(rename(serialize = "publicKeyPem", deserialize="publicKeyPem"))]
    pub public_key_pem: String
}

/// A structure for returning
/// and receiving info about
/// an actor's attachments.
#[derive(Serialize, Deserialize)]
pub struct ActorAttachment{
    pub attachment_type: ActorAttachmentType,
    pub name: String,
    pub value: String
}

/// A structure for returning
/// and receiving info about
/// an actor.
#[derive(Serialize, Deserialize)]
pub struct Actor {
    #[serde(rename(serialize = "id", deserialize="id"))]
    pub user_id: String,
    #[serde(rename(serialize = "type", deserialize="type"))]
    pub user_type: UserType, // In the db.
    pub following: String, // In the db from "ActorFollower"
    pub followers: String, // In the db from "ActorFollower"
    pub inbox: String, // Generic.
    pub outbox: String, // Generic.
    pub featured: String, // Generic. (is_featured for activity)
    #[serde(rename(serialize = "featuredTags", deserialize="featuredTags"))]
    pub featured_tags: Vec<ActivityHashTag>, // Not in the db.
    #[serde(rename(serialize = "preferredUsername", deserialize="preferredUsername"))]
    pub preferred_username: String, // In the db.
    pub name: String, // In the db.
    pub summary: String, // in the db.
    pub url: String, // Generic.
    #[serde(rename(serialize = "manuallyApprovesFollowers", deserialize="manuallyApprovesFollowers"))]
    pub manually_approves_followers: bool, // In the db.
    pub discoverable: bool, // In the db.
    pub indexable: bool, // In the db.
    pub published: String, // In the db.
    pub memorial: bool, // In the db.
    #[serde(rename(serialize = "attributionDomains", deserialize="attributionDomains"))]
    pub attribution_domains: Vec<String>, // In the db.
    pub attachments: Vec<ActorAttachment>, // In the db.
    pub image: ActorMedia, // In the db.
    pub icon: ActorMedia, // In the db.
    pub endpoints: HashMap<String, String> // Generic.
}

/// A structure for returning
/// and receiving info about
/// attachments to an activity.
#[derive(Serialize, Deserialize)]
pub struct ActivityAttachment {
    pub file_type: ActorAttachmentType,
    #[serde(rename(serialize = "mediaType", deserialize="mediaType"))]
    pub media_type: String,
    pub url: String,
    pub name: String,
}

/// A structure for returning
/// and receiving info about
/// different types of tags.
#[derive(Serialize, Deserialize)]
pub struct ActivityHashTag {
    #[serde(rename(serialize = "type", deserialize="type"))]
    pub tag_type: String,
    pub href: String,
    pub name: String
}

/// A structure for returning
/// and receiving info about
/// interactions on an activity.
#[derive(Serialize, Deserialize)]
pub struct ActivityInteraction {
    pub id: String,
    pub interaction_type: CollectionType,
    #[serde(rename(serialize = "totalItems", deserialize="totalItems"))]
    pub total_items: usize
}

/// A structure for returning
/// and receiving info about
/// an activity.
#[derive(Serialize, Deserialize)]
pub struct Activity {
    #[serde(rename(serialize = "id", deserialize="id"))]
    pub activity_id: String, // In the db. Done.
    #[serde(rename(serialize = "type", deserialize="type"))]
    pub activity_type: ActivityEntityType, // In the db. Done.
    pub summary: String, // In the db. // Done.
    #[serde(rename(serialize = "inReplyTo", deserialize="inReplyTo"))]
    pub in_reply_to: String, // In the db.
    pub published: String, // in the db.
    pub url: String, // generic.
    #[serde(rename(serialize = "attributedTo", deserialize="attributedTo"))]
    pub attributed_to: String, // In the db.
    pub to: Vec<String>, // Generic
    pub cc: Vec<String>, // Generic
    pub sensitive: bool, // in the db.    
    pub conversation: String, // "tag:mastodon.social,2025-02-23:objectId=928605497:objectType=Conversation"
    pub content: String,
    pub attachment: Vec<ActivityAttachment>, // TO DO
    pub tag: Vec<ActivityHashTag>, // TO DO.
    pub replies: ActivityReplies, // TO DO.
    pub likes: ActivityInteraction, // TO DO.
    pub shares: ActivityInteraction // TO DO.
}

/// A structure for returning
/// and receiving info about
/// a public activity.
#[derive(Serialize, Deserialize)]
pub struct StreamedActivity {
    pub id: String,
    pub activity_type: ActivityType, // TO DO.
    pub actor: String,
    pub published: String,
    pub to: Vec<String>, // TO DO.
    pub cc: Vec<String>, // TO DO.
    pub object: Activity
}

/// A structure for returning
/// and receiving info about
/// replies to an activity.
#[derive(Serialize, Deserialize)]
pub struct ActivityReplies {
    #[serde(rename(serialize = "id", deserialize="id"))]
    pub collection_pointer: String,
    #[serde(rename(serialize = "type", deserialize="type"))]
    pub entity_type: CollectionType,
}

/// A structure to receive and send
/// WebFinger info about an actor.
#[derive(Serialize, Deserialize)]
pub struct ActorWebFinger{
    pub subject: String,
    pub aliases: Vec<String>,
    pub links: Vec<WebFingerResource>
}

/// A structure to receive and send
/// WebFinger resources about an 
/// actor.
#[derive(Serialize, Deserialize)]
pub struct WebFingerResource{
    pub rel: String,
    #[serde(rename(serialize = "type", deserialize="type"))]
    pub res_type: Option<String>,
    pub href: Option<String>,
    pub template: Option<String>
}

/// Declaring a structure
/// to provide feedback as a 
/// JSON response on whether
/// an operation was successful
/// or not.
pub struct StatusResponse{
    pub status: bool
}
