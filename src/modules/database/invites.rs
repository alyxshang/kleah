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

/// Importing the model for storing
/// invite codes in the database.
use crate::modules::units::models::InviteCode;

/// Attempts to write a created
/// invite code to the database.
/// If this operation is successful,
/// an instance of the "InviteCode"
/// structure is returned. If this operation
/// fails, an error is returned.
pub async fn write_invite_code(
    code: &str,
    admin_id: &str,
    pool: &Pool<Postgres>
) -> Result<InviteCode, KleahErr>{
    let code_id: String = hash_string_sha(code);
    let code: InviteCode = InviteCode{
        code_id: code_id.clone(),
        code: code.to_string(),
        admin_id: admin_id.to_string()
    };
    let _insert_op = match query!(
        "INSERT INTO invite_codes (code_id, code, admin_id) VALUES ($1, $2, $3)",
        code.code_id,
        code.code,
        code.admin_id

    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<InviteCode, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let written_code: InviteCode = match get_code_by_id(&code_id, pool)
        .await
    {
        Ok(written_code) => written_code,
        Err(e) => return Err::<InviteCode, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(written_code)
}

/// Attempts to retrieve
/// an invite code given the
/// code's ID. If this operation
/// is successful, an instance of
/// the "InviteCode" structure
/// is returned. If the operation
/// fails, an error is returned.
pub async fn get_code_by_id(
    code_id: &str,
    pool: &Pool<Postgres>
) -> Result<InviteCode, KleahErr> {
    let code_obj: InviteCode = match query_as!(
        InviteCode,
        "SELECT * FROM invite_codes WHERE code_id = $1", 
        code_id
    )
        .fetch_one(pool)
        .await 
    {
        Ok(code_obj) => code_obj,
        Err(e) => return Err::<InviteCode, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(code_obj)
}

/// Attempts to delete an
/// invite code written to the
/// database. If this is successful,
/// an empty function is returned. If
/// this fails, an error is returned.
pub async fn delete_code(
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
