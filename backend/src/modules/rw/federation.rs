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

/// Importing the macro
/// from the "sqlx" crate
/// to execute SQL queries.
use sqlx::query_as;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::Postgres;

/// Importing the structure that models
/// a follower relationship between two
/// Kleah users.
use crate::models::KleahUserFollows;
use crate::responses::SubscriptionLink;
use crate::utils::get_webfinger_info_from_other_instance;
use crate::responses::WebFingerInfo;
use crate::responses::WebFingerLink;
use crate::responses::UserKeyActor;
use crate::responses::ActorFollows;
use crate::responses::UserIcon;
/// Importing this crate's
/// structure to catch and
/// handle errors.
use crate::KleahErr;

/// Importing the structure
/// that will serve as a response
/// for creating an actor
/// from a user for ActivityPub.
use crate::responses::Actor;

/// Importing the "Charm"
/// structure to work with charms
/// and explicitly declare
/// them.
use crate::models::KleahUser;

/// Importing the function to retrieve a 
/// user by a token associated with them.
use crate::rw_utils::get_user_by_handle;

use crate::rw_utils::get_user_by_id;

use crate::rw_utils::get_instance_hostname;

// URL: /users/{username}/
// Method: GET
pub async fn gather_actor_info(
    username: &String,
    pool: &Pool<Postgres>    
) -> Result<Actor, KleahErr> {
    let user: KleahUser = match get_user_by_handle(username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Actor, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let hostname: String = match get_instance_hostname(pool).await {
        Ok(hostname) => hostname,
        Err(e) => return Err::<Actor, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let users_following: Vec<KleahUserFollows> = match query_as!(KleahUserFollows, "SELECT * FROM user_follows WHERE follower = $1", &user.user_id)
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<Actor, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let users_followed: Vec<KleahUserFollows> = match query_as!(KleahUserFollows, "SELECT * FROM user_follows WHERE followee = $1", &user.user_id)
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<Actor, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut followers: Vec<String> = Vec::new();
    let mut following: Vec<String> = Vec::new();
    for kleah_user_follow in users_following {
        let user: KleahUser = match get_user_by_id(&kleah_user_follow.follower, pool).await {
            Ok(user) => user,
            Err(e) => return Err::<Actor, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let formatted: String = format!("{}/users/{}", &user.host, &user.username);
        following.push(formatted);
    }
    for kleah_user_follow in users_followed {
        let user: KleahUser = match get_user_by_id(&kleah_user_follow.followee, pool).await {
            Ok(user) => user,
            Err(e) => return Err::<Actor, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let formatted: String = format!("{}/users/{}", &user.host, &user.username);
        followers.push(formatted);
    }
    let actor_followers: ActorFollows = ActorFollows{
        list_type: "OrderedCollection".to_string(),
        total_items: followers.len(),
        items: followers
    };
    let actor_following: ActorFollows = ActorFollows{
        list_type: "OrderedCollection".to_string(),
        total_items: following.len(),
        items: following
    };
    let user_icon: UserIcon = UserIcon{
        icon_type: "Image".to_string(),
        icon_url: format!("{}/{}", &hostname, &user.avatar_url)
    };
    let user_id: String = format!("{}/users/{}", &hostname, &user.username);
    let inbox_url: String = format!("{}/users/{}/inbox", &hostname, &user.username);
    let outbox_url: String = format!("{}/users/{}/inbox", &hostname, &user.username);
    let user_key: UserKeyActor = UserKeyActor{
        user_id: format!("{}#main-key",&user_id),
        owner: user_id.clone(),
        public_key_pem: user.pub_key
    };
    let actor: Actor = Actor {
        user_id: user_id.clone(), 
        entity_type: "Person".to_string(),
        summary: user.user_description,
        name: user.display_name,
        icon: user_icon,
        outbox: outbox_url,
        inbox: inbox_url,
        followers: actor_followers,
        following: actor_following,
        public_key: user_key 
    };
    Ok(actor)
}

// URL: /users/{username}/followers
// Method: GET
pub async fn gather_actor_following(
    username: &String,
    pool: &Pool<Postgres>
) -> Result<ActorFollows, KleahErr> {
    let user: KleahUser = match get_user_by_handle(username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<ActorFollows, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut users: Vec<String> = Vec::new();
    let users_followed: Vec<KleahUserFollows> = match query_as!(KleahUserFollows, "SELECT * FROM user_follows WHERE follower = $1", &user.user_id)
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<ActorFollows, KleahErr>(KleahErr::new(&e.to_string()))
    };
    for kleah_user_follow in users_followed {
        let user: KleahUser = match get_user_by_id(&kleah_user_follow.follower, pool).await {
            Ok(user) => user,
            Err(e) => return Err::<ActorFollows, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let formatted: String = format!("{}/users/{}", &user.host, &user.username);
        users.push(formatted);
    }
    let actor_followers: ActorFollows = ActorFollows{
        list_type: "OrderedCollection".to_string(),
        total_items: users.len(),
        items: users
    };
    Ok(actor_followers)
}

// URL: /users/{username}/following
// Method: GET?
pub async fn gather_actor_followers(
    username: &String,
    pool: &Pool<Postgres>
) -> Result<ActorFollows, KleahErr> {
    let user: KleahUser = match get_user_by_handle(username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<ActorFollows, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut users: Vec<String> = Vec::new();
    let users_following_actor: Vec<KleahUserFollows> = match query_as!(KleahUserFollows, "SELECT * FROM user_follows WHERE followee = $1", &user.user_id)
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<ActorFollows, KleahErr>(KleahErr::new(&e.to_string()))
    };
    for kleah_user_follow in users_following_actor {
        let user: KleahUser = match get_user_by_id(&kleah_user_follow.follower, pool).await {
            Ok(user) => user,
            Err(e) => return Err::<ActorFollows, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let formatted: String = format!("{}/users/{}", &user.host, &user.username);
        users.push(formatted);
    }
    let actor_followers: ActorFollows = ActorFollows{
        list_type: "OrderedCollection".to_string(),
        total_items: users.len(),
        items: users
    };
    Ok(actor_followers)
}

// URL: /users/{username}#main-key
// Crate: https://crates.io/crates/pgp
pub async fn user_public_key(
    username: &String, 
    pool: &Pool<Postgres>
) -> Result<String, KleahErr> {
    todo!("Implement this.")    
}

// URL: /.well-known/webfinger?resource=acct:username@host
// Method: GET
pub async fn get_webfinger_info(
    username: &String, 
    host: &String,
    pool: &Pool<Postgres>
) -> Result<WebFingerInfo, KleahErr>{
    let current_hostname: String = match get_instance_hostname(pool).await {
        Ok(current_hostname) => current_hostname,
        Err(e) => return Err::<WebFingerInfo, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if host == &current_hostname {
        let subject: String = format!("acct:{}@{}", username, host);
        let mut aliases: Vec<String> = Vec::new();
        aliases.push(format!("https://{}/@{}", host, username));
        aliases.push(format!("https://{}/users/{}", host, username));
        let profile_page: WebFingerLink = WebFingerLink{
            rel: "http://webfinger.net/rel/profile-page".to_string(),
            link_type: "text/html".to_string(),
            href: format!("https://{}/@{}", host, username)
        };
        let activity_page: WebFingerLink = WebFingerLink{
            rel: "self".to_string(),
            link_type: "application/activity+json".to_string(),
            href: format!("https://{}/users|{}", host, username)
        };
        let user: KleahUser = match get_user_by_handle(username, pool).await {
            Ok(user) => user,
            Err(e) => return Err::<WebFingerInfo, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let pfp_url: String = format!("https://{}/{}", &current_hostname, user.avatar_url);
        let content_type: String;
        if pfp_url.ends_with("jpg"){ content_type = "jpeg".to_string() }
        else { content_type = "png".to_string() }
        let pfp_page: WebFingerLink = WebFingerLink{
            rel: "http://webfinger.net/rel/avatar".to_string(),
            link_type: format!("image/{}", content_type),
            href: pfp_url
        };
        let subscribe_page: SubscriptionLink = SubscriptionLink{
            rel: "http://ostatus.org/schema/1.0/subscribe".to_string(),
            template: format!("https://{}/authorize-follow?acct={{uri}}", &current_hostname),
        };
        let mut pages: Vec<WebFingerLink> = Vec::new();  
        pages.push(profile_page);
        pages.push(activity_page);
        pages.push(pfp_page);
        //pages.push(subscribe_page);
        let webfinger_info: WebFingerInfo = WebFingerInfo{
            subject: subject,
            aliases: aliases,
            links: pages
        };
        Ok(webfinger_info)
    }
    else {
        let webfinger_info: WebFingerInfo = match get_webfinger_info_from_other_instance(host, username).await{
            Ok(webfinger_info) => webfinger_info,
            Err(e) => return Err::<WebFingerInfo, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(webfinger_info)
    }
}