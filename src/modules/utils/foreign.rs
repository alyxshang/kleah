/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Client"
/// structure to submit 
/// POST requests.
use reqwest::Client;

/// Importing the "Response"
/// structure for explicit 
/// typing.
use reqwest::Response;

/// Importing this crate's
/// structure for catching
/// and handling errors.
use super::err::KleahErr;

/// Importing the function
/// to deserialize JSON
/// into a Rust data
/// structure.
use serde_json::from_str;

/// Importing the "ACCEPT"
/// enum to sepcify which sort
/// of content one wants to 
/// accept when sending a request.
use reqwest::header::ACCEPT;

/// Importing the "Actor"
/// structure to retrieve
/// the inbox URL of an 
/// actor.
use crate::responses::Actor;

/// Importing the standard structure
/// for maps from the standard
/// library.
use std::collections::HashMap;

/// Importing the structure to
/// send and receive data on
/// an actor's activities.
use crate::responses::Activity;

/// Importing the "to_string_pretty"
/// function to serialize a Rust
/// data structure into a JSON
/// string.
use serde_json::to_string_pretty;

/// Importing the "CONTENT_TYPE"
/// enum to sepcify which sort
/// of content one wants to 
/// receive when sending a request.
use reqwest::header::CONTENT_TYPE;

/// Importing the "ActorWebFinger"
/// structure to fetch all relevant
/// links on an actor.
use crate::responses::ActorWebFinger;

/// Importing the "AUTHORIZATION"
/// enum to send authorized requests.
use reqwest::header::AUTHORIZATION;

/// This function attempts to
/// fetch an actor's WebFinger
/// data and read the actor's
/// ActivityPub URL. If this
/// operation fails, an error
/// is returned.
pub async fn fetch_actor_url(
    host: &str, 
    username: &str
) -> Result<String, KleahErr>{
    let client = Client::new();
    let std_rel: String = "http://ostatus.org/schema/1.0/subscribe".to_string();
    let webfinger_url: String = format!(
        "https://{}/.well-known/webfinger?resource=acct:{}@{}",
        host,
        username,
        host
    );
    let resp: Response = match client
        .get(webfinger_url)
        .header(ACCEPT, "application/activity+json")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await 
    {
        Ok(resp) => resp,
        Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let text: String = match resp.text().await{
        Ok(text) => text,
        Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let deserialized: ActorWebFinger = match from_str(&text){
        Ok(deserialized) => deserialized,
        Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: String = String::from("");
    for link in deserialized.links{
        if link.rel == std_rel{}
        else {
            let res_type: String = match link.res_type{
                Some(res_type) => res_type,
                None=> return Err::<String, KleahErr>(KleahErr::new(&"Unknown type.".to_string()))
            };
            let href: String = match link.href{
                Some(href) => href,
                None=> return Err::<String, KleahErr>(KleahErr::new(&"Unknown refererral link.".to_string()))
            };
            if res_type == "application/activity+json".to_string(){
                result = href;
            }
            else {}
        }
    }
    if &result == ""{
        let e: String = "Could not fetch actor url.".to_string();
        return Err::<String, KleahErr>(KleahErr::new(&e.to_string()));
    }
    Ok(result)
}

/// This function attempts 
/// to fetch an actor's
/// ActivityPub inbox URL.
/// If this operation fails, 
/// an error is returned.
pub async fn get_actor_inbox(
    actor_url: &str
) -> Result<String, KleahErr>{
    let client = Client::new();
    let resp: Response = match client
        .get(actor_url)
        .header(ACCEPT, "application/activity+json")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await 
    {
        Ok(resp) => resp,
        Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let text: String = match resp.text().await{
        Ok(text) => text,
        Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let deserialized: Actor = match from_str(&text){
        Ok(deserialized) => deserialized,
        Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let inbox_field: String = deserialized.inbox;
    Ok(inbox_field)
}

/// This function attempts 
/// to post to an actor's
/// ActivityPub inbox URL
/// in a "raw" manner.
/// If this operation fails, 
/// an error is returned.
pub async fn post_to_inbox(
    inbox_url: &str,
    activity: &Activity,
    server_key: &str,
) -> Result<bool, KleahErr>{
    let client = Client::new();
    let serialized: String = match to_string_pretty(activity){
        Ok(serialized) => serialized,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let resp: Response = match client
        .post(inbox_url)
        .header(ACCEPT, "application/activity+json")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, server_key)
        .json(&serialized)
        .send()
        .await 
    {
        Ok(resp) => resp,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: bool = false;
    if resp.status() == 200{
        result = true;
    }
    else {}
    Ok(result)
}

/// This function attempts 
/// to post to an actor's
/// ActivityPub inbox URL
/// If this operation fails, 
/// an error is returned.
pub async fn post_activity_to_inbox(
    host: &str, 
    username: &str,
    server_key: &str,
    activity: &Activity,
) -> Result<bool, KleahErr>{
    let actor_url: String = match fetch_actor_url(host, username).await {
        Ok(actor_url) => actor_url,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let inbox_url: String = match get_actor_inbox(&actor_url).await {
        Ok(actor_url) => actor_url,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let posted_status: bool = match post_to_inbox(&inbox_url, activity, &server_key).await {
        Ok(actor_url) => actor_url,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(posted_status)
}