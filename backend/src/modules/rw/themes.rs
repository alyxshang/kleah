/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

use bcrypt::DEFAULT_COST;
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

/// Importing the 
/// "hash" function
/// to hash strings.
use bcrypt::hash;

/// Importing the macro
/// from the "sqlx" crate
/// to execute SQL queries.
use sqlx::query_as;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::Postgres;


/// Importing the structure to
/// submit a payload for saving
/// a user-generated theme.
use crate::payloads::CreateThemePayload;

/// Importing the structure to
/// submit a payload for deleting
/// a user-generated theme.
use crate::payloads::DeleteThemePayload;

use crate::responses::StatusResponse;
use crate::time::get_time;
use crate::KleahErr;
use crate::models::UserTheme;
use crate::models::KleahUser;
use crate::rw_utils::get_user_from_token;

pub async fn write_theme(
    payload: &CreateThemePayload,
    pool: &Pool<Postgres>
) -> Result<UserTheme, KleahErr> {
    let to_hash: String = format!("{}:{}", &payload.theme_name, get_time());
    let hashed: String = match hash(&to_hash, DEFAULT_COST){
        Ok(hashed) => hashed,
        Err(e) => return Err::<UserTheme, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let user: KleahUser = match get_user_from_token(&payload.api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<UserTheme, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let new_theme: UserTheme = UserTheme{
        theme_id: hashed,
        theme_owner: user.user_id,
        theme_name: payload.theme_name,
        primary_color: payload.primary_color,
        accent_color: payload.accent_color
    };
    let _insert_op = match sqlx::query!(
        "INSERT INTO user_themes (theme_id, theme_owner, theme_name, primary_color, accent_color) VALUES ($1, $2, $3, $4, $5)",
        new_theme.theme_id,
        new_theme.theme_owner,
        new_theme.theme_name,
        new_theme.primary_color,
        new_theme.accent_color
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<UserTheme, KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(new_theme)
}

pub async fn wipe_theme(
    payload: &DeleteThemePayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, KleahErr>{
    let user: KleahUser = match get_user_from_token(&payload.api_token, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let user_theme: UserTheme = match query_as!(UserTheme, "SELECT * FROM user_themes WHERE theme_owner = $1", user.user_id)
        .fetch_one(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<UserTimeline, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let _wipe_op: () = match query!("DELETE FROM user_themes WHERE theme_id = $1", user_theme.theme_id)
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, KleahErr>(KleahErr::new(&e.to_string()))
        };
    Ok(StatusResponse{status:0})
}