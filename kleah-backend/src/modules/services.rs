/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "get"
/// decorator to make a service
/// that accepts "GET" requests.
use actix_web::get;

/// Importing the "post"
/// decorator to make a service
/// that accepts "POST" requests.
use actix_web::post;

/// Importing the "Result"
/// enum for Actix Web services.
use actix_web::Result;

/// Importing the "Data"
/// structure to register
/// persistent app data.
use actix_web::web::Data;

/// Importing this crate's
/// error structure.
use super::err::KleahErr;

/// Importing the "Json"
/// structure to return JSON
/// responses.
use actix_web::web::Json;

/// Importing the data structure
/// for creating a public actor.
use super::models::Actor;

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

/// Importing the function
/// to parse a host string.
use super::utils::parse_host;

/// Importing the "WebFingerLInk"
/// structure to dynamically build
/// webfinger link resources.
use super::units::WebFingerLink;

use super::models::UserAPIToken;

/// Importing the function to create
/// a new user in the database.
use super::database::create_user;

/// Importing the data structure that
/// allows a user to create an API token.
use super::units::LoginUserPayload;

/// Importing the data structure that
/// allows a user to destroy an API
/// token.
use super::units::LogoutUserPayload;

/// Importing the structure
/// to capture and deserialize
/// URL parameters.
use super::units::WebFingerResource;

/// Importing the data structure
/// for submitting a payload for
/// creating a new user.
use super::units::SignUpUserPayload;

/// Importing the structure to return
/// a WebFinger response.
use super::units::WebFingerResponse;

/// Importing the function to create a new
/// API token for a user.
use super::database::create_api_token;

/// Importing the function to retrieve
/// an actor given their handle.
use super::database::get_actor_by_name;

/// A service function to
/// create a user. If the operation
/// is successful, a JSON response
/// of the created public actor
/// is returned. In any other case,
/// an error is returned.
#[post("/api/signup")]
pub async fn signup_user(
    payload: Json<SignUpUserPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, KleahErr>{
    let new_user: Actor = match create_user(
        &payload.username,
        &payload.password,
        &false,
        &payload.email_addr,
        &payload.display_name,
        &payload.user_type,
        &payload.default_primary,
        &payload.default_secondary,
        &payload.default_tertiary,
        &data.pool
    ).await {
        Ok(new_user) => new_user,
        Err(e) => return Err::<HttpResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(HttpResponse::Ok().json(new_user))
}

#[get("/.well-known/webfinger?resource=acct:{username}@{domain}")]
pub async fn webfinger_discovery(
    info: Path<WebFingerResource>,
    data: Data<AppData>
) -> Result<HttpResponse, KleahErr> {
    let parsed_path: WebFingerResource = info.into_inner();
    let username: String = parsed_path.username;
    let domain: String = parsed_path.domain;
    let actor: Actor = match get_actor_by_name(
        &username, 
        &data.pool
    ).await {
        Ok(actor) => actor,
        Err(e) =>   return Err::<HttpResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let nkd_host: String = match parse_host(&actor.host){
        Ok(nkd_host) => nkd_host,
        Err(e) =>   return Err::<HttpResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    if actor.user_id == username && domain == nkd_host {
        let acct_name: String = format!(
            "acct:{}@{}",
            actor.user_id,
            nkd_host
        );
        let profile_url: String = format!(
            "{}/@{}",
            actor.host,
            actor.user_id
        );
        let act_url: String = format!(
            "{}/users/{}",
            actor.host,
            actor.user_id
        );
        let profile_wfl: WebFingerLink = WebFingerLink{
            rel: "http://webfinger.net/rel/profile-page".to_string(),
            content_type: Some("text/html".to_string()),
            href: Some(profile_url.clone()),
            template: None
        };
        let act_wfl: WebFingerLink = WebFingerLink{
            rel: "self".to_string(),
            content_type: Some("text/html".to_string()),
            href: Some(act_url.clone()),
            template: None
        };
        let tmpl_wfl: WebFingerLink = WebFingerLink{
            rel: "http://ostatus.org/schema/1.0/subscribe".to_string(),
            content_type: None,
            href: None,
            template: Some(format!("{}/authorize_interaction?uri={{uri}}", actor.host))
        };
        let mut str_arr: Vec<String> = Vec::new();
        str_arr.push(profile_url.clone());
        str_arr.push(act_url.clone());
        let mut wfl_arr: Vec<WebFingerLink> = Vec::new();
        wfl_arr.push(profile_wfl);
        wfl_arr.push(act_wfl);
        wfl_arr.push(tmpl_wfl);
        let wfr: WebFingerResponse = WebFingerResponse{
            subject: acct_name,
            aliases: str_arr,
            links: wfl_arr
        };
        Ok(HttpResponse::Ok().json(wfr))
    }
    else {
        let e: String = format!(
            "The actor \"{}@{}\" does not exist on this server.",
            username,
            domain
        );
        Err::<HttpResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    }
}

#[post("/api/token/create")]
pub async fn login_user(
    payload: Json<LoginUserPayload>,
    data: Data<AppData>
) -> Result<HttpResponse, KleahErr>{
    let token: UserAPIToken = match create_api_token(
        &payload.username, 
        &payload.password, 
        &data.pool
    ).await {
        Ok(token) => token,
        Err(e) => return Err::<HttpResponse, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(HttpResponse::Ok().json(token))
}

/* TIMELINE START */
#[post("/timeline/following")]
pub async fn user_time_line_following_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/timeline/global")]
pub async fn user_time_line_global_service(
) -> Result<HttpResponse, KleahErr>{}
/* TIMELINE END */


/* USER START */
#[get("/user/profile")]
pub async fn user_profile_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/user/files/private")]
pub async fn user_files_private_service(
) -> Result<HttpResponse, KleahErr>{}

#[get("/user/files/public")]
pub async fn user_files_public_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/user/follow")]
pub async fn user_follow_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/user/unfollow")]
pub async fn user_unfollow_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/user/edit/email")]
pub async fn user_edit_email_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/user/edit/password")]
pub async fn user_edit_password_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/user/edit/colors/primary")]
pub async fn user_edit_pc_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/user/edit/colors/secondary")]
pub async fn user_edit_sc_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/user/edit/colors/tertiary")]
pub async fn user_edit_tc_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/user/edit/name")]
pub async fn user_edit_name_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/user/edit/bio")]
pub async fn user_edit_bio_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/user/edit/indexable")]
pub async fn user_edit_indexable_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/user/edit/memorial")]
pub async fn user_edit_memorial_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/user/edit/findable")]
pub async fn user_edit_findable_service(
) -> Result<HttpResponse, KleahErr>{}
/* USER END */


/* INSTANCE START */
#[post("/instance/edit/name")]
pub async fn edit_instance_name_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/instance/edit/primary")]
pub async fn edit_instance_primary_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/instance/edit/secondary")]
pub async fn edit_instance_secondary_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/instance/edit/tertiary")]
pub async fn edit_instance_tertiary_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/instance/edit/host")]
pub async fn edit_instance_host_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/instance/edit/smtp/host")]
pub async fn edit_instance_smtp_host_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/instance/edit/smtp/pass")]
pub async fn edit_instance_smtp_pass_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/instance/edit/admin")]
pub async fn edit_instance_admin_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/instance/edit/description")]
pub async fn edit_instance_description_service(
) -> Result<HttpResponse, KleahErr>{}
/* INSTANCE END */


/* NOTES START */
#[post("/notes/boost")]
pub async fn note_boost_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/notes/unboost")]
pub async fn note_unboost_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/notes/share")]
pub async fn note_share_service(
) -> Result<HttpResponse, KleahErr>{}

#[get("/notes/like")]
pub async fn note_like_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/notes/unlike")]
pub async fn note_unlike_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/notes/create")]
pub async fn note_create_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/notes/delete")]
pub async fn note_delete_service(
) -> Result<HttpResponse, KleahErr>{}

#[post("/notes/edit")]
pub async fn note_edit_service(
) -> Result<HttpResponse, KleahErr>{}
/* NOTES END */
