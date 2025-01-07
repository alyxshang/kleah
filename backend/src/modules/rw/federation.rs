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

