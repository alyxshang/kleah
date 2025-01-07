/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Serialize"
/// trait to derive it and 
/// serialize Rust structures
/// as JSON.
use serde::Serialize;

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

/// A data structure that
/// returns all relevant
/// information on a user's
/// Kleah account for Webfinger.
#[derive(Serialize)]
pub struct WebfingerResponse {
    pub subject: String,
    pub links: Vec<WebfingerLink>
}

/// A data structure that
/// returns all relevant
/// information on a webfinger
/// link for a Kleah user's
/// Webfinger info.
#[derive(Serialize)]
pub struct WebfingerLink{
    pub rel: String,
    #[serde(rename(serialize = "type"))]
	pub link_type: String,
	pub href: String,
}

/// A data structure that
/// models an actor for
/// a user for ActivityPub.
#[derive(Serialize)]
pub struct Actor {
  #[serde(rename(serialize = "id"))]
  pub user_id: String,
  #[serde(rename(serialize = "type"))]
  pub entity_type: String,
  pub name: String,
  pub summary: String,
  pub icon: UserIcon,
  pub outbox: String,
  pub inbox: String,
  pub followers:ActorFollows,
  pub following: ActorFollows,
  #[serde(rename(serialize = "publicKey"))]
  pub public_key: UserKeyActor
}

#[derive(Serialize)]
pub struct UserIcon {
  #[serde(rename(serialize = "type"))]
  pub icon_type: String,
  #[serde(rename(serialize = "url"))]
  pub icon_url: String,
}

#[derive(Serialize)]
pub struct UserKeyActor{
  #[serde(rename(serialize = "id"))]
  pub user_id: String,
  pub owner: String,
  #[serde(rename(serialize = "publicKeyPem"))]
  pub public_key_pem: String,
}

// Shape
/*
{
        "id": "https://yourplatform.com/users/alyxshang",
        "type": "Person",
        "preferredUsername": "alyxshang",
        "name": "Alyx Shang",
        "summary": "A description or bio of the user",
        "icon": {
          "type": "Image",
          "url": "https://yourplatform.com/avatars/alyxshang.png"
        },
        "outbox": "https://yourplatform.com/users/alyxshang/outbox",
        "inbox": "https://yourplatform.com/users/alyxshang/inbox",
        "followers": "https://yourplatform.com/users/alyxshang/followers",
        "following": "https://yourplatform.com/users/alyxshang/following",
        "publicKey": {
          "id": "https://yourplatform.com/users/alyxshang#main-key",
          "owner": "https://yourplatform.com/users/alyxshang",
          "publicKeyPem": "-----BEGIN PUBLIC KEY----- ... -----END PUBLIC KEY-----"
        }
      }
*/

#[derive(Serialize)]
pub struct ActorFollows {
  #[serde(rename(serialize = "type"))]
  pub list_type: String,
  pub total_items: usize,
  pub items: Vec<String>
}

// Shape for followers.
/*
{
  "type": "OrderedCollection",
  "totalItems": 5,
  "items": [
    "https://yourplatform.com/users/follower1",
    "https://yourplatform.com/users/follower2"
  ]
}
*/

// Shape for following.
/*
{
  "type": "OrderedCollection",
  "totalItems": 5,
  "items": [
    "https://yourplatform.com/users/followee1",
    "https://yourplatform.com/users/followee2"
  ]
}
*/

pub struct UserAccountPublicKey{

}

// Shape:
/*
{
   "@context":[
      "https://www.w3.org/ns/activitystreams",
      "https://w3id.org/security/v1",
      {
         "manuallyApprovesFollowers":"as:manuallyApprovesFollowers",
         "toot":"http://joinmastodon.org/ns#",
         "featured":{
            "@id":"toot:featured",
            "@type":"@id"
         },
         "featuredTags":{
            "@id":"toot:featuredTags",
            "@type":"@id"
         },
         "alsoKnownAs":{
            "@id":"as:alsoKnownAs",
            "@type":"@id"
         },
         "movedTo":{
            "@id":"as:movedTo",
            "@type":"@id"
         },
         "schema":"http://schema.org#",
         "PropertyValue":"schema:PropertyValue",
         "value":"schema:value",
         "discoverable":"toot:discoverable",
         "suspended":"toot:suspended",
         "memorial":"toot:memorial",
         "indexable":"toot:indexable",
         "attributionDomains":{
            "@id":"toot:attributionDomains",
            "@type":"@id"
         },
         "focalPoint":{
            "@container":"@list",
            "@id":"toot:focalPoint"
         }
      }
   ],
   "id":"https://mastodon.social/users/alyxshang",
   "type":"Person",
   "following":"https://mastodon.social/users/alyxshang/following",
   "followers":"https://mastodon.social/users/alyxshang/followers",
   "inbox":"https://mastodon.social/users/alyxshang/inbox",
   "outbox":"https://mastodon.social/users/alyxshang/outbox",
   "featured":"https://mastodon.social/users/alyxshang/collections/featured",
   "featuredTags":"https://mastodon.social/users/alyxshang/collections/tags",
   "preferredUsername":"alyxshang",
   "name":"🖤𝓐𝓛𝓨𝓧 - 天权🖤",
   "summary":"\u003cp\u003e🖤 𝕬𝖓𝖈𝖎𝖊𝖓𝖙 𝖈𝖗𝖊𝖆𝖙𝖎𝖛𝖊 𝖘𝖔𝖚𝖑.\u003c/p\u003e",
   "url":"https://mastodon.social/@alyxshang",
   "manuallyApprovesFollowers":false,
   "discoverable":true,
   "indexable":true,
   "published":"2024-12-29T00:00:00Z",
   "memorial":false,
   "publicKey":{
      "id":"https://mastodon.social/users/alyxshang#main-key",
      "owner":"https://mastodon.social/users/alyxshang",
      "publicKeyPem":"-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAna7rcVY1OOjfDto4i154\nz4AGaQYcOQkRQ1U4UzCNvc+CTfBDDxO69sNjIotE2YQW0b8xhFJ2BEapbNCpGgDH\nNABv+QVlwOWXBUTx6FjGnoXtWr1g0j19V/LpGMiSd4NhVfgqYn198sx7K3z3e5u0\n1mD7glzEEMvCwrcBuPkeXaaLkqLa+gOx80q+IiXnEcjfDfJOaHOgsK3MwrxhOy8U\nUH+PmfAZVRYf7crOJBxgH28EFqjmdAOwAUgCkts4riRTVMq+WnKqNc6DzJTMgcNf\nAQW5eKFePW3itDdUmI8keqX7+iI+Q4MfTw1P8+A/ic+nLUuALA+Mtsll7SgUb+yS\nKwIDAQAB\n-----END PUBLIC KEY-----\n"
   },
   "tag":[
      
   ],
   "attachment":[
      {
         "type":"PropertyValue",
         "name":"GitHub",
         "value":"\u003ca href=\"https://github.com/alyxshang\" target=\"_blank\" rel=\"nofollow noopener me\" translate=\"no\"\u003e\u003cspan class=\"invisible\"\u003ehttps://\u003c/span\u003e\u003cspan class=\"\"\u003egithub.com/alyxshang\u003c/span\u003e\u003cspan class=\"invisible\"\u003e\u003c/span\u003e\u003c/a\u003e"
      },
      {
         "type":"PropertyValue",
         "name":"Website",
         "value":"\u003ca href=\"https://alyxshang.boo/\" target=\"_blank\" rel=\"nofollow noopener me\" translate=\"no\"\u003e\u003cspan class=\"invisible\"\u003ehttps://\u003c/span\u003e\u003cspan class=\"\"\u003ealyxshang.boo/\u003c/span\u003e\u003cspan class=\"invisible\"\u003e\u003c/span\u003e\u003c/a\u003e"
      }
   ],
   "endpoints":{
      "sharedInbox":"https://mastodon.social/inbox"
   },
   "icon":{
      "type":"Image",
      "mediaType":"image/png",
      "url":"https://files.mastodon.social/accounts/avatars/113/737/404/937/317/251/original/2770fbaf57f2ac60.png"
   },
   "image":{
      "type":"Image",
      "mediaType":"image/png",
      "url":"https://files.mastodon.social/accounts/headers/113/737/404/937/317/251/original/c9813ff99fbf68b8.png"
   }
}
*/