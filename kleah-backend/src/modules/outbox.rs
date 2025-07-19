use reqwest::Client;
use serde::Serialize;
use reqwest::Response;
use serde_json::Value;
use serde::Deserialize;
use super::err::KleahErr;
use serde_json::from_str;
use std::collections::HashMap;

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
pub async fn get_stream_url(
    host: &String,
    user: &String
) -> Result<WebFingerResponse, KleahErr> {
    let client: Client = Client::new();
    let url: String = format!(
        "{}/.well-known/webfinger?resource=acct:{}@{}",
        host,
        user,
        host
    );
    let response: Response = match client.get(url)
        .header("Accept", "application/activity+json")
        .header("Content-Type", "application/activity+json")
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => return Err::<WebFingerResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let text: String = match response.text().await{
        Ok(text) => text,
        Err(e) => return Err::<WebFingerResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let wfr: WebFingerResponse = match from_str(&text){
        Ok(wfr) => wfr,
        Err(e) => return Err::<WebFingerResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(wfr)
}

pub fn get_actor_info_url(
    webfinger_res: &WebFingerResponse,
) -> Result<String, KleahErr> {
    let mut res: Vec<String> = Vec::new();
    for link in &webfinger_res.links {
        if link.content_type == Some("application/activity+json".to_string()){
            match &link.href{
                Some(url) => res.push(url.to_string()),
                None => {}
            };
        }
    }
    if res.len() == 1{
        Ok(res[0].clone())
    }
    else {
        Err::<String, KleahErr>(
            KleahErr::new(
                "Actor info could not be retrieved."
            )
        )
    }
}

pub async fn get_actor_apub_info(
    activity_url: &String,
) -> Result<ActorWebFingerInfo, KleahErr> {
    let client: Client = Client::new();
    let response: Response = match client.get(activity_url)
        .header("Accept", "application/activity+json")
        .header("Content-Type", "application/activity+json")
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => return Err::<ActorWebFingerInfo, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let text: String = match response.text().await{
        Ok(text) => text,
        Err(e) => return Err::<ActorWebFingerInfo, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let awfi: ActorWebFingerInfo = match from_str(&text){
        Ok(awfi) => awfi,
        Err(e) => return Err::<ActorWebFingerInfo, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(awfi)
}

pub fn get_actor_inbox_url(
    actor_info: &ActorWebFingerInfo
) -> String {
    actor_info.inbox.clone()
}

pub fn get_actor_outbox_url(
    actor_info: &ActorWebFingerInfo
) -> String {
    actor_info.outbox.clone()
}

pub async fn get_actor_outbox_info(
    outbox_url: String
) -> Result<ActorOutBoxInfo, KleahErr> {
    let client: Client = Client::new();
    let response: Response = match client.get(outbox_url)
        .header("Accept", "application/activity+json")
        .header("Content-Type", "application/activity+json")
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => return Err::<ActorOutBoxInfo, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let text: String = match response.text().await{
        Ok(text) => text,
        Err(e) => return Err::<ActorOutBoxInfo, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let aoi: ActorOutBoxInfo = match from_str(&text){
        Ok(aoi) => aoi,
        Err(e) => return Err::<ActorOutBoxInfo, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(aoi)
}

pub async fn crawl_actor_outbox(
    outbox_info: &ActorOutBoxInfo
) -> Result<Vec<Activity>, KleahErr> {
    let mut res: Vec<Activity> = Vec::new();
    Ok(res)
}
