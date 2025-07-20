/// Importing the "get"
/// decorator to make a service
/// that accepts "GET" requests.
use actix_web::get;

/// Importing the "Data"
/// structure to register
/// persistent app data.
use actix_web::web::Data;

/// importing the "Path"
/// entity to capture
/// URL parameters.
use actix_web::web::Path;

/// Importing the data structure
/// to have one SQL pool for
/// allservice functions.
use super::units::AppData;

/// Importing the function
/// to return a HTTP response.
use actix_web::HttpResponse;

use reqwest::Client;
use serde::Serialize;
use reqwest::Response;
use serde_json::Value;
use serde::Deserialize;
use super::err::KleahErr;
use serde_json::from_str;
use std::collections::HashMap;
use super::units::WebFingerResource;
use super::units::WebFingerLink;
use super::units::WebFingerResponse;
use super::units::ActorWebFingerInfo;
use super::units::ActorOutBoxInfo;
use super::units::Activity;

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

#[get("/{username}/outbox")]
pub async fn actor_outbox(
    username: Path<String>,
    data: Data<AppData>
) -> Result<HttpResponse, KleahErr>{
    let username: String = username.into_inner();
    todo!("Implement!")
}
