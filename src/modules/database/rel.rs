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

/// Importing the function
/// to generate a hash from
/// a string.
use crate::utils::hash_string_sha;

/// Importing the "KleahErr"
/// structure to catch and
/// handle errors.
use crate::modules::utils::err::KleahErr;

/// Importing the data structure for modelling
/// a block-relationship in the database.
use crate::modules::units::models::ActorBlock;

/// Importing the data structure for modelling
/// a follower-relationship in the database.
use crate::modules::units::models::ActorFollower;

pub async fn create_follow(
    follower_id: &str,
    followee_id: &str,
    pool: &Pool<Postgres>
) -> Result<ActorFollower, KleahErr>{
    let rel_id: String = hash_string_sha(
        &format!("{}{}", follower_id, followee_id)
    );
    let follower: ActorFollower = ActorFollower{
        rel_id: rel_id,
        actor_id: followee_id,
        follower_id: follower_id        
    };
    let insert_op: () = match query!(
        "INSERT INTO actor_followers (rel_id, actor_id, follower_id) VALUES ($1, $2, $3)",
        follower.rel_id,
        follower.actor_id,
        follower.follower_id

    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<InviteCode, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    }; 
    Ok(inser_op)
}

pub async fn create_block(
    actor_id: &str,
    blocked_actor: &str,
    pool: &Pool<Postgres>
) -> Result<ActorFollower, KleahErr>{
    let rel_id: String = hash_string_sha(
        &format!("{}{}", follower_id, followee_id)
    );
    let follower: ActorBlock = ActorBlock{
        rel_id: rel_id,
        actor_id: followee_id,
        blocked_actor: follower_id        
    };
    let insert_op: () = match query!(
        "INSERT INTO actor_blocks (rel_id, actor_id, follower_id) VALUES ($1, $2, $3)",
        follower.rel_id,
        follower.actor_id,
        follower.follower_id

    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<InviteCode, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    }; 
    Ok(inser_op)
}
