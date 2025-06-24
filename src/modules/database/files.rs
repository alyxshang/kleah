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

use crate::err::KleahErr;
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
use crate::modules::units::models::ActorFile;

use crate::modules::database::token::get_actor_by_token;

pub async fn create_actor_file(
    api_token: &str,
    path: &str,
    extension: &str,
    pool: &Pool<Postgres>
) -> Result<ActorFile, KleahErr>{
    let user: Actor = get_actor_from_token(&api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<ActorFile, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let id: String = hash_string_sha(
        format!(
            "{}{}", 
            path, 
            TimeNow()::new().to_string()
        )
    );
    let actor_file: ActorFile = ActorFile{
        media_id: id,
        user_id: user.user_id,
        file_path: path,
        file_extension: extension
    };
    let _insert_op = match query!(
        "INSERT INTO actor_files (media_id, user_id, file_path, file_extension) VALUES ($1, $2, $3, $4)",
        actor_file.media_id,
        actor_file.user_id,
        actor_file.file_path,
        actor_file.file_extension
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<InviteCode, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let written: ActorFile = match get_file_by_id(id).await {
        Ok(written) => written,
        Err(e) => return Err::<ActorFile, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(written)
}

pub async fn get_file_by_id(
    media_id: &str,
    pool: &Pool<Postgres>
) -> Result<ActorFile, KleahErr> {
    let file_obj: ActorFile = match query_as!(
        ActorFile,
        "SELECT * FROM actor_files WHERE media_id = $1", 
        media_id
    )
        .fetch_one(pool)
        .await 
    {
        Ok(file_obj) => file_obj,
        Err(e) => return Err::<ActorFile, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(file_obj)
}

pub async fn delete_file(
    api_token: &str,
    code_id: &str,
    pool: &Pool<Postgres>
) -> Result<(), KleahErr>{
    let written_code: InviteCode = match get_code_by_id(&code_id, pool)
        .await
    {
        Ok(written_code) => written_code,
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let del_op: () = match query!(
        "DELETE FROM invite_codes WHERE code_id = $1", 
        written_code.code_id
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
