/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Pool"
/// structure to use a pool
/// of connections.
use sqlx::Pool;

/// Importing the "query"
/// macro to execute queries
/// that return nothing.
use sqlx::query;

/// Importing the "query_as"
/// macro to execute queries
/// that return something.
use sqlx::query_as;

/// Importing the "Postgres"
/// structure for explicit 
/// typing.
use sqlx::postgres::Postgres;

/// Importing the "TimeNow" structure to get
/// the current time.
use crate::modules::units::trans::TimeNow;

/// Importing the function
/// to generate a hash from
/// a string.
use crate::modules::utils::hash_string_sha;

/// Importing the structure
/// from the database for
/// casting the type of a
/// private actor explicitly.
use crate::modules::units::models::PrivateActor;

/// Importing the structure
/// for reading and writing info
/// about API tokens to and from the
/// database.
use crate::modules::units::models::UserAPIToken;

/// Importing the "KleahErr"
/// structure to catch and
/// handle errors.
use crate::modules::utils::err::KleahErr;

/// Importing the function to retrieve info
/// on a private actor given their username 
/// from the database.
use crate::modules::database::users::get_private_actor_by_username;

pub async fn create_api_token(
    username: &str,
    password: &str,
    pool: &Pool<Postgres>
) -> Result<UserAPIToken, KleahErr>{
    let p_actor: PrivateActor = match get_private_actor_by_username(
        username,
        pool
    ).await {
        Ok(p_actor) => p_actor,
        Err(e) => return Err::<UserAPIToken, KleahErr>{
            KleahErr::new(&e.to_string())
        }
    };
    let hashed: String = hash_string_sha(
        format!(
            "{}{}", 
            p_actor.user_id, 
            TimeNow::new().to_string()
        )
    );
    let new_token: UserAPIToken = UserAPIToken{
        token: hashed,
        user_id: p_actor.user_id
    };
    let _insert_op = match query!(
        "INSERT INTO tokens (token, user_id) VALUES ($1, $2)",
        new_token.token,
        new_token.user_id,

    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<InviteCode, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let written: UserAPIToken = match get_token_obj(new_token.token, pool).await{
        Ok(written) => written,
        Err(e) => return Err::<UserAPIToken, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(written)
}

pub async fn get_token_obj(
    token: &str,
    pool: &Pool<Postgres>
) -> Result<UserAPIToken, KleahErr>{
    let token_obj: UserAPIToken = match query_as!(
        UserAPIToken,
        "SELECT * FROM tokens WHERE token = $1", 
        token
    )
        .fetch_one(pool)
        .await 
    {
        Ok(token) => token,
        Err(e) => return Err::<InviteCode, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(token_obj)
}

pub async fn get_private_actor_by_token(
    token: &str,
    pool: &Pool<Postgres>
) -> Result<PrivateActor, KleahErr>{
    let token_obj: UserAPIToken = match get_token_obj(new_token.token, pool).await{
        Ok(token_obj) => token_obj,
        Err(e) => return Err::<UserAPIToken, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let p_actor_obj: PrivateActor = match query_as!(
        PrivateActor,
        "SELECT * FROM private_actors WHERE user_id = $1", 
        token_obj.user_id
    )
        .fetch_one(pool)
        .await 
    {
        Ok(p_actor_obj) => p_actor_obj,
        Err(e) => return Err::<PrivateActor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(p_actor_obj)    
}

pub async fn get_actor_by_token(
    token: &str,
    pool: &Pool<Postgres>
) -> Result<Actor, KleahErr>{
    let token_obj: UserAPIToken = match get_token_obj(new_token.token, pool).await{
        Ok(token_obj) => token_obj,
        Err(e) => return Err::<UserAPIToken, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let actor_obj: Actor = match query_as!(
        Actor,
        "SELECT * FROM actors WHERE user_id = $1", 
        token_obj.user_id
    )
        .fetch_one(pool)
        .await 
    {
        Ok(actor_obj) => actor_obj,
        Err(e) => return Err::<Actor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(actor_obj)    
}

pub async fn del_token(
    token: &str,
    pool: &Pool<Postgres>
) -> Result<(), KleahErr>{
    let del_op: () = match query!(
        "DELETE FROM tokens WHERE token = $1", 
        token
    )
        .execute(pool)
        .await 
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(del_op)
}
