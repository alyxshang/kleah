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
/// "query" macro
/// from the "sqlx"
/// crate to execute
/// queries.
use sqlx::query;

/// Importing the
/// "query" macro
/// from the "sqlx"
/// crate to execute
/// queries with a 
/// certain return type.
use sqlx::query_as;

/// Importing this crate's
/// error structure.
use crate::err::KleahErr;

/// Importing the model for
/// storing information
/// about an actor.
use crate::models::Actor;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// Importing the model for
/// storing confidential
/// information about an
/// actor.
use crate::models::PrivateActor;

/// Importing the structure to store
/// info on actor keys.
use crate::modules::units::trans::ActorKeys;

/// Importing the function to hash
/// a string and encrypt it.
use crate::modules::utils::utils::hash_string;

/// Importing the function to generate pair
/// of public and private keys for an actor.
use crate::modules::utils::utils::generate_keys;

/// Importing the function to hash
/// a string.
use crate::modules::utils::utils::hash_string_sha;

/// Attempts to save confidential
/// information about an actor
/// in the database. If this
/// operation is successful,
/// an instance of the "PrivateActor"
/// structure is returned. If this
/// operation fails, an error is
/// returned.
pub async fn create_private_actor(
    email: &str,
    username: &str,
    password: &str,
    is_admin: bool,
    pool: &Pool<Postgres>
) -> Result<PrivateActor, KleahErr>{
    let pwd: String = match hash_string(password){
        Ok(pwd) => pwd,
        Err(e) => return Err::<PrivateActor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let user_id: String = hash_string_sha(username);
    let keys: ActorKeys = match generate_keys(){
        Ok(keys) => keys,
        Err(e) => return Err::<PrivateActor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let p_actor: PrivateActor = PrivateActor{
        user_id: user_id.clone(),
        email: email.to_string(),
        verified: false,
        privileged: is_admin,
        private_key: keys.private,
        public_key: keys.public,
        user_password: pwd,
    };
    let _insert_op = match query!(
        "INSERT INTO private_actors (user_id, email, verified, privileged, private_key, public_key, user_password) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        p_actor.user_id,
        p_actor.email,
        p_actor.verified,
        p_actor.privileged,
        p_actor.private_key,
        p_actor.public_key,
        p_actor.user_password

    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<PrivateActor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let written: PrivateActor = match get_private_actor_by_id(&user_id, pool).await{
        Ok(written) => written,
        Err(e) => return Err::<PrivateActor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(written)
}

/// Attempts to retrieve private
/// information on an actor given the
/// code's ID. If this operation
/// is successful, an instance of
/// the "PrivateActor" structure
/// is returned. If the operation
/// fails, an error is returned.
pub async fn get_private_actor_by_id(
    id: &str, 
    pool: &Pool<Postgres>
) -> Result<PrivateActor, KleahErr>{
    let actor_obj: PrivateActor = match query_as!(
        PrivateActor,
        "SELECT * FROM private_actors WHERE user_id = $1", 
        id
    )
        .fetch_one(pool)
        .await 
    {
        Ok(code_obj) => code_obj,
        Err(e) => return Err::<PrivateActor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(actor_obj)
}

/// Attempts to retrieve
/// information on an actor given the
/// code's ID. If this operation
/// is successful, an instance of
/// the "PrivateActor" structure
/// is returned. If the operation
/// fails, an error is returned.
pub async fn get_actor_by_id(
    id: &str, 
    pool: &Pool<Postgres>
) -> Result<Actor, KleahErr>{
    let actor_obj: Actor = match query_as!(
        Actor,
        "SELECT * FROM actors WHERE user_id = $1", 
        id
    )
        .fetch_one(pool)
        .await 
    {
        Ok(code_obj) => code_obj,
        Err(e) => return Err::<Actor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(actor_obj)
}

/// Attempts to save 
/// information about an actor
/// in the database. If this
/// operation is successful,
/// an instance of the "PrivateActor"
/// structure is returned. If this
/// operation fails, an error is
/// returned.
pub async fn create_actor(
    private_actor: &PrivateActor,
    host: &str,
    user_type: &str,
    default_banner: &str,
    default_pfp: &str,
    pool: &Pool<Postgres>
) -> Result<PrivateActor, KleahErr>{
    if user_type != "person" || user_type != "bot"{
        let e: String = "Wrong user type receveied.".to_string();
        return Err::<PrivateActor, KleahErr>(
            KleahErr::new(&e.to_string())
        );
    }
    let actor: PrivateActor = PrivateActor{
        user_id: private_actor.user_id,
        host: host.to_string(),
        user_type: user_type,
        preferred_username: private_actor.username,
        display_name: "".to_string(),
        summary: "".to_string(),
        manually_approves_followers: false,
        discoverable: false,
        indexable: bool,
        published: "".to_string(),
        memorial: false
        banner_id: default_banner,
        pfp_id: default_pfp
    };
    let _insert_op = match query!(
        "INSERT INTO actors (user_id, host, user_type, preferred_username, display_name, summary, manually_approves_followers, discoverable, indexable, published, memorial, banner_id, pfp_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
        actor.user_id,
        actor.host,
        actor.user_type,
        actor.preferred_username,
        actor.display_name,
        actor.summary,
        actor.manually_approves_followers,
        actor.discoverable,
        actor.indexable,
        actor.published,
        actor.memorial,
        actor.banner_id,
        actor.pfp_id,

    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<Actor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let written: Actor = match get_actor_by_id(&user_id, pool).await{
        Ok(written) => written,
        Err(e) => return Err::<Actor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(written)
}

pub async fn update_display_name(
    api_token: &str,
    new_value: &str,
    pool: &Pool<Postgres>
) -> Result<(), KleahErr>{
    let user: PriivateActor = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let update_op: () = match query!(
        "UPDATE actors SET display_name = $1 WHERE user_id = $2",
        new_value, 
        user.user_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(update_op)
}

pub async fn update_summary(
    api_token: &str,
    new_value: &str,
    pool: &Pool<Postgres>
) -> Result<(), KleahErr>{
    let user: PriivateActor = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let update_op: () = match query!(
        "UPDATE actors SET summary = $1 WHERE user_id = $2",
        new_value, 
        user.user_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(update_op)
}

pub async fn update_maf(
    api_token: &str,
    new_value: &bool,
    pool: &Pool<Postgres>
) -> Result<(), KleahErr>{
    let user: PriivateActor = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let update_op: () = match query!(
        "UPDATE actors SET manually_approves_followers = $1 WHERE user_id = $2",
        new_value, 
        user.user_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(update_op)
}

pub async fn update_discoverable(
    api_token: &str,
    new_value: &bool,
    pool: &Pool<Postgres>
) -> Result<(), KleahErr>{
    let user: PriivateActor = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let update_op: () = match query!(
        "UPDATE actors SET discoverable = $1 WHERE user_id = $2",
        new_value, 
        user.user_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(update_op)
}

pub async fn update_discoverable(
    api_token: &str,
    new_value: &bool,
    pool: &Pool<Postgres>
) -> Result<(), KleahErr>{
    let user: PriivateActor = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let update_op: () = match query!(
        "UPDATE actors SET indexable = $1 WHERE user_id = $2",
        new_value, 
        user.user_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(update_op)
}

pub async fn update_memorial(
    api_token: &str,
    new_value: &bool,
    pool: &Pool<Postgres>
) -> Result<(), KleahErr>{
    let user: PriivateActor = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let update_op: () = match query!(
        "UPDATE actors SET memorial = $1 WHERE user_id = $2",
        new_value, 
        user.user_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(update_op)
}

pub async fn update_banner_id(
    api_token: &str,
    new_value: &str,
    pool: &Pool<Postgres>
) -> Result<(), KleahErr>{
    let user: PriivateActor = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let update_op: () = match query!(
        "UPDATE actors SET banner_id = $1 WHERE user_id = $2",
        new_value, 
        user.user_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(update_op)
}

pub async fn update_pfp_id(
    api_token: &str,
    new_value: &str,
    pool: &Pool<Postgres>
) -> Result<(), KleahErr>{
    let user: PriivateActor = match get_user_from_token(api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    let update_op: () = match query!(
        "UPDATE actors SET pfp_id = $1 WHERE user_id = $2",
        new_value, 
        user.user_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), CleoErr>(CleoErr::new(&e.to_string()))
    };
    Ok(update_op)
}

pub async fn delete_actor(
    api_token: &str,
    pool: &Pool<Postgres>
) -> Result<(), KleahErr>{
    let actor: Actor = match get_actor_from_token(api_token, pool).await {
        Ok(actor) => actor,
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let private_actor: PrivateActor = match get_private_actor_from_token(
        api_token, 
        pool
    ).await {
        Ok(private_actor) => private_actor,
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let _actor_del_op: () = match query!(
        "DELETE FROM actors WHERE user_id = $1", 
        actor.user_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let del_op: () = match query!(
        "DELETE FROM private_actors WHERE user_id = $1", 
        private_actor.user_id
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(())
}

pub async fn get_private_actor_by_username(
    username: &str,
    pool: &Pool<Postgres>
) -> Result<PrivateActor, KleahErr>{
    let actor_obj: PrivateActor = match query_as!(
        PrivateActor,
        "SELECT * FROM private_actors WHERE username = $1", 
        username
    )
        .fetch_one(pool)
        .await 
    {
        Ok(actor_obj) => actor_obj,
        Err(e) => return Err::<PrivateActor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(actor_obj)
}
