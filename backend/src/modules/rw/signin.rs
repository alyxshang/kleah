
/// Importing the
/// "Pool" structure
/// from the "sqlx" crate
/// to make a pool for
/// database connections.
use sqlx::Pool;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::Postgres;

use sqlx::query;

use crate::models::APIToken;
use crate::models::KleahUser;
use crate::payloads::CreateUserTokenPayload;
use crate::responses::StatusResponse;
use crate::tokens::create_new_token;
use crate::tokens::wipe_token;
use crate::payloads::LoginTokenPayload;
use crate::payloads::DiscardLoginTokenPayload;

/// Importing this crate's
/// structure to catch and
/// handle errors.
use crate::KleahErr;

use super::rw_utils::get_user_by_id;
use super::rw_utils::get_user_by_handle;
use super::rw_utils::get_user_from_token;

pub async fn login_user_from_db(
    payload: &LoginTokenPayload,
    pool: &Pool<Postgres>
) -> Result<APIToken, KleahErr> {
    let user: KleahUser = match get_user_by_handle(payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<APIToken, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let token_payload: CreateUserTokenPayload = CreateUserTokenPayload {
        user_id: user.user_id,
        password: payload.password,
        can_change_pwd: true,
        can_change_username: true,
        can_post_charms: true,
        can_delete_user: true,
        can_change_email: true,
    };
    let created: APIToken = match create_new_token(&token_payload, pool).await {
        Ok(created) => created,
        Err(e) => return Err::<APIToken, KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(created)
}

pub async fn logout_user_from_db(
    payload: &DiscardLoginTokenPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr> {
    let _wipe_op: () = match query!("DELETE FROM api_tokens WHERE token = $1", payload.api_token)
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let status: StatusResponse = StatusResponse{ status: 0 };
        Ok(status)
}