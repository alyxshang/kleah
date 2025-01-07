
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

/// Importing the macro
/// from the "sqlx" crate
/// to execute SQL queries.
use sqlx::query;

/// Importing this crate's
/// structure to catch and
/// handle errors.
use crate::KleahErr;

/// Importing the "APIToken"
/// structure to work with API
/// tokens and explicitly declare
/// them.
use crate::models::APIToken;

/// Importing the function to create
/// a new API token in the database.
use crate::tokens::create_new_token;

/// Importing the "StatusResponse"
/// structure to return information
/// on operational success.
use crate::responses::StatusResponse;

/// Importing the structure to submit
/// a new payload for logging a user
/// into a Kleah instance.
use crate::payloads::LoginTokenPayload;

/// Importing the structure to submit a new
/// payload for creating a new API token.
use crate::payloads::CreateUserTokenPayload;

/// Importing the structure to submit
/// a new payload for logging a user
/// out of a Kleah instance.
use crate::payloads::DiscardLoginTokenPayload;


/// Attempts to create a new API token
/// for logging in an performaing actions
/// when the user logs in to their Kleah 
/// interface.
pub async fn login_user_from_db(
    payload: &LoginTokenPayload,
    pool: &Pool<Postgres>
) -> Result<APIToken, KleahErr> {
    let token_payload: CreateUserTokenPayload = CreateUserTokenPayload {
        username: payload.username.clone(),
        password: payload.password.clone(),
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

/// Attempts to wipe the API token used
/// for logging in an performaing actions
/// when the user logs out of their Kleah 
/// interface.
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