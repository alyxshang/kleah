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

/// Importing this crate's
/// structure to catch and
/// handle errors.
use crate::KleahErr;

/// Importing the "Charm"
/// structure to work with charms
/// and explicitly declare
/// them.
use crate::models::Charm;

/// Importing the "APIToken"
/// structure to work with API
/// tokens and explicitly declare
/// them.
use crate::models::APIToken;

/// Importing the "KleahUser"
/// structure to work with Kleah
/// users and explicitly declare
/// them.
use crate::models::KleahUser;

/// Attempts to fetch the user with the given handle from the database.
/// If this operation succeeds, an instance of the "KleahUser" structure is
/// returned. If this operation fails, an error is returned. This function
/// is NOT utilized in any API routes.
pub async fn get_user_by_id(
    user_id: &String,
    pool: &Pool<Postgres>
) -> Result<KleahUser, KleahErr> {
    let users: Vec<KleahUser> = match query_as!(KleahUser, "SELECT * FROM users")
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: Vec<KleahUser> = Vec::new();
    for user in users {
        if &user.user_id == user_id {
            result.push(user);
        }
        else {}
    }
    if result.len() == 1{
        Ok(result[0].clone())
    }
    else{
        let e: String = format!("User \"{}\" does not exist.", &user_id);
        Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// This function attempts to get the
/// user associated with the supplied API
/// token. If this operation succeeds, an
/// instance of the user object whom the token belongs
/// to is supplied. If the operation fails, an error
/// is returned. This function
/// is NOT utilized in any API routes.
pub async fn get_user_from_token(
    api_token: &String, 
    pool: &Pool<Postgres>
) -> Result<KleahUser, KleahErr> {
    let api_tokens: Vec<APIToken> = match query_as!(APIToken, "SELECT * FROM api_tokens")
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut user_id: String = "".to_string();
    for token in api_tokens {
        if &token.token == api_token && token.is_active {
            user_id = token.user_id;
        }
        else {}
    }
    if user_id == "".to_string(){
        let e: String = "No user with the specified API token found.".to_string();
        Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
    }
    else {
        let user: KleahUser = match get_user_by_id(&user_id, pool).await {
            Ok(user) => user,
            Err(e) => return Err::<KleahUser, KleahErr>(KleahErr::new(&e.to_string()))
        };
        Ok(user)
    }
}

/// This function attempts to get
/// an instance of the "APIToken"
/// structure from the database,
/// given the user's API token.
/// If the operation fails, an error
/// is returned. This function
/// is NOT utilized in any API routes.
pub async fn get_api_token_object(
    token: &String, 
    pool: &Pool<Postgres>
) -> Result<APIToken, KleahErr>{
    let tokens: Vec<APIToken> = match query_as!(APIToken, "SELECT * FROM api_tokens")
        .fetch_all(pool)
        .await
    {
        Ok(tokens) => tokens,
        Err(e) => return Err::<APIToken, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: Vec<APIToken> = Vec::new();
    for stored in tokens {
        if &stored.token == token {
            result.push(stored);
        }
        else {}
    }
    if result.len() == 1 {
        Ok(result[0].clone())
    }
    else {
        let e: String = format!("Token \"{}\" not found.", token);
        Err::<APIToken, KleahErr>(KleahErr::new(&e.to_string()))
    }
}

/// This function attempts to retrieve a charm
/// given the supplied ID. If this operation fails,
/// an error is returned.
pub async fn get_charm_by_id(
    charm_id: &String, 
    pool: &Pool<Postgres>
) -> Result<Charm, KleahErr>{
    let tokens: Vec<Charm> = match query_as!(Charm, "SELECT * FROM charms")
        .fetch_all(pool)
        .await
    {
        Ok(tokens) => tokens,
        Err(e) => return Err::<Charm, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let mut result: Vec<Charm> = Vec::new();
    for stored in tokens {
        if &stored.charm_id == charm_id {
            result.push(stored);
        }
        else {}
    }
    if result.len() == 1 {
        Ok(result[0].clone())
    }
    else {
        let e: String = format!("Charm \"{}\" not found.", charm_id);
        Err::<Charm, KleahErr>(KleahErr::new(&e.to_string()))
    }
}