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
use sqlx::query;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::Postgres;

/// Importing this crate's
/// structure to catch and
/// handle errors.
use crate::KleahErr;

/// Importing the "KleahUser"
/// structure to work with users
/// and explicitly declare
/// them.
use crate::models::KleahUser;

/// Importing the "InviteCode"
/// structure to work with
/// creating invite codes.
use crate::models::InviteCode;

/// Importing the function to generate
/// a sequence of random characters.
use crate::utils::generate_chars;

/// Importing the "StatusResponse"
/// structure to return information
/// on operational success.
use crate::responses::StatusResponse;

/// Importing the function to retrieve a 
/// user by a token associated with them.
use crate::rw_utils::get_user_from_token;

/// Importing the structure for submitting
/// a payload for creating new invite codes
/// for an instance.
use crate::payloads::CreateInviteCodePayload;

/// Importing the structure for submitting
/// a payload for deleting new invite codes
/// for an instance.
use crate::payloads::DeleteInviteCodePayload;

/// This function to store a new invite code into
/// the database. If the owner of the supplied API token
/// is the Tianquan, this operation will succeed. An instance
/// of the "InviteCode" structure will be returned. If the 
/// operation fails, an error will be returned.
pub async fn create_invite_code(
    payload: &CreateInviteCodePayload,
    pool: Pool<Postgres>
) -> Result<InviteCode, KleahErr> {
    let invite_code: String = generate_chars(12);
    let user: KleahUser = match get_user_from_token(&payload.api_token, &pool).await {
        Ok(user) => user,
        Err(e) => return Err::<InviteCode, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if user.is_admin && user.is_active{
        let new_invite: InviteCode = InviteCode{
            invite_code: invite_code,
            user_id: user.user_id
        };
        let _insert_op = match query!(
            "INSERT INTO invite_codes (user_id, invite_code) VALUES ($1, $2)",
            new_invite.user_id,
            new_invite.invite_code,
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<InviteCode, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(new_invite)
    }
    else {
        let e: String = "The user with the requesting API token is not an administrator.".to_string();
        Err::<InviteCode, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// Attempts to delete a charm for a user given 
/// one of their API tokens and charm ID. If this operation 
/// succeeds, an instance of  the "StatusResponse" 
/// structure is returned with a status code of 0. 
/// If this operation fails, an error is returned 
/// or an instance of the "StatusResponse" structure 
/// with the status code of 1 is returned.
pub async fn wipe_invite_code(
    payload: &DeleteInviteCodePayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr> {
    let _wipe_op: () = match query!("DELETE FROM invite_codes WHERE invite_code = $1", payload.invite_code)
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let status: StatusResponse = StatusResponse{ status: 0 };
    Ok(status)
}