/*
Jade by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// "Pool" structure
/// from the "sqlx" crate
/// to make a pool for
/// database connections.
use sqlx::Pool;

/// Importing the "hash"
/// function so strings can
/// be hashed.
use bcrypt::hash;

/// Importing the "verify"
/// function so 
/// hashed strings can
/// be verified.
use bcrypt::verify;

/// Importing this crate's
/// error structure.
use super::err::JadeErr;

/// Importing the default
/// hashing speed for hashing
/// strings.
use bcrypt::DEFAULT_COST;

/// Importing the function
/// to get the current time
/// to get proper timestamps.
use super::time::get_time;

/// Importing the stucture that
/// contains information on
/// Jade users.
use super::units::JadeUser;

/// Importing the stucture that
/// contains information on
/// the mood of a Jade user.
use super::units::JadeMood;

/// Importing the structure
/// for storing information
/// on a user's API tokens.
use super::units::APIToken;

/// Importing the function
/// to send an email.
use super::email::send_email;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// Importing the function
/// to store a file.
use super::utils::save_file;

/// Importing the structure
/// to save information
/// on a user's profile.
use super::units::ProfileInfo;

/// Importing the structure
/// to log a user into Jade.
use super::units::LoginPayload;

/// Importing the structure that
/// helps store user-uploaded files.
use super::units::JadeUserFile;

/// Importing the structure
/// to see whether an operation
/// was successful or not.
use super::units::StatusResponse;

/// Importing the data structure
/// to save information about
/// the follower-followee 
/// relationship of Jade users.
use super::units::JadeUserFollows;

/// Importing the structure
/// to conduct operations that only
/// require a user's token.
use super::units::TokenOnlyPayload;

/// Importing the structure
/// to conduct operations on
/// a user's moods.
use super::units::MoodActionPayload;

/// Importing the structure
/// to conduct the creation of a 
/// user account.
use super::units::CreateUserPayload;

/// Importing the structure to show
/// active and inactive mooods.
use super::units::UserMoodsResponse;

/// Importing the payload containing
/// information needed for un/following
/// Jade users.
use super::units::UserFollowPayload;

/// Importing the structure
/// to conduct the creation of a 
/// new API token.
use super::units::CreateTokenPayload;

/// Importing the structure
/// to conduct the deletion of a 
/// new API token.
use super::units::DeleteTokenPayload;

/// Importing the structure
/// to conduct operations on
/// a user's account info.
use super::units::ChangeEntityPayload;

/// Importing the structure
/// to conduct operations that only
/// require a user's handle.
use super::units::UsernameOnlyPayload;

/// Importing the structure to 
/// retrieve information on active
/// API tokens.
use super::units::UserAPITokensPayload;

/// Import the structure to store
/// session tokens for Jade users.
use super::units::JadeUserSessionToken;

/// This function attempts to get the
/// user associated with the supplied API
/// token. If this operation succeeds, an
/// instance of the user whom the token belongs
/// to is supplied. If the operation fails, an error
/// is returned.
pub async fn get_user_from_token(
    api_token: &String, 
    pool: &Pool<Postgres>
) -> Result<JadeUser, JadeErr> {
    let api_tokens: Vec<APIToken> = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens")
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let mut username: String = "".to_string();
    for token in api_tokens {
        if &token.token == api_token && token.is_active {
            username = token.username;
        }
        else {}
    }
    if username == "".to_string(){
        let e: String = "No user with the specified API token found.".to_string();
        Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    }
    else {
        let user: JadeUser = match get_user_by_handle(&username, pool).await {
            Ok(user) => user,
            Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
        };
        Ok(user)
    }
}

/// This function attempts to store
/// the supplied bytes in a file on disk
/// and writes a new instance of the "JadeUserFile"
/// structure in the database. If this operation fails,
/// an error is returned.
pub async fn store_file(
    file: &Vec<u8>,
    api_token: &String, 
    name: &String,
    pool: &Pool<Postgres>,
    instance_url: &String,
    folder_name: &String,
) -> Result<JadeUserFile, JadeErr>{
    let user: JadeUser = match get_user_from_token(api_token,pool).await {
        Ok(user) => user,
        Err(e) => return Err::<JadeUserFile, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let hashed_id: String = match hash(format!("{}:{}", &name, get_time()), DEFAULT_COST){
        Ok(hashed_id) => hashed_id,
        Err(e) => return Err::<JadeUserFile, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let file_path: String = match save_file(file, &user.username, folder_name, name).await {
        Ok(file_path) => file_path,
        Err(e) => return Err::<JadeUserFile, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let file_url: String = format!("{}/{}", instance_url, file_path);
    let new_file: JadeUserFile = JadeUserFile{
        file_id: hashed_id,
        file_name: name.to_owned(),
        username: user.username,
        file_path: file_url
    };
    let _insert_op = match sqlx::query!(
        "INSERT INTO user_files (file_id, username, file_name, file_path) VALUES ($1, $2, $3, $4)",
        new_file.file_id,
        new_file.username,
        new_file.file_name,
        new_file.file_path,
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<JadeUserFile, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(new_file)  
}

/// This function attempts to verify the email
/// the user has submitted. If the operation succeeds,
/// a boolean "true" is returned. If the operation fails,
/// an error is returned or a boolean "false" is returned.
pub async fn verify_user_email(
    email_token: &String,
    pool: &Pool<Postgres>
) -> Result<bool, JadeErr> {
    let mut result: bool = false;
    let users: Vec<JadeUser> = match sqlx::query_as!(JadeUser, "SELECT * FROM users")
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<bool, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let mut user_vec: Vec<JadeUser> = Vec::new();
    for user in users {
        if &user.email_token == email_token {
            result = true;
            user_vec.push(user);
        }
        else {}
    }
    let hashed_time: String = match hash(get_time(), DEFAULT_COST){
        Ok(hashed_time) => hashed_time,
        Err(e) => return Err::<bool, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if user_vec.len() == 1 {}
    else {
        let e: String = "No user with the specified token found.".to_string();
        return Err::<bool, JadeErr>(JadeErr::new(&e.to_string()))
    }
    let user: JadeUser = user_vec[0].clone();
    let _update_op_active: () = match sqlx::query!("UPDATE users SET is_active = $1 WHERE username = $2", true, user.username)
            .execute(pool)
            .await
    {
        Ok(_feedback) => result = true,
        Err(e) => return Err::<bool, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let _update_token: () = match sqlx::query!("UPDATE users SET email_token = $1 WHERE username = $2", hashed_time, user.username)
            .execute(pool)
            .await
    {
        Ok(_feedback) => result = true,
        Err(e) => return Err::<bool, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(result)
}

/// Attempts to create a new user with the given payload.
/// If this operation succeeds, an instance of the "JadeUser" structure is
/// returned. If this operation fails, an error is returned.
pub async fn write_user(
    payload: &CreateUserPayload,
    pool: &Pool<Postgres>,
    smtp_server: &String
) -> Result<JadeUser, JadeErr> {
    let hashed_pwd = match hash(payload.password.clone(), DEFAULT_COST){
        Ok(hashed) => hashed,
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let hashed_email_token = match hash(&format!("{}{}{}", &payload.username, &payload.email, get_time()), DEFAULT_COST){
        Ok(hashed) => hashed,
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let hashed_email = match hash(&payload.email, DEFAULT_COST){
        Ok(hashed) => hashed,
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let new_user: JadeUser = JadeUser{
        username: payload.username.clone(),
        avatar_url: "".to_string(),
        display_name: payload.display_name.clone(),
        email: hashed_email.clone(),
        pwd: hashed_pwd,
        email_token: hashed_email_token.clone(),
        is_active: false,
    };
    let _insert_op = match sqlx::query!(
        "INSERT INTO users (username, display_name, avatar_url, email, pwd, email_token, is_active) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        new_user.username,
        new_user.display_name,
        new_user.avatar_url,
        new_user.email,
        new_user.pwd,
        new_user.email_token,
        new_user.is_active
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let email_sub: String = format!("Confirm your email address, {}.", &payload.username);
    let from_addr: String = format!("Jade <noreply@{}>", smtp_server);
    let to_addr: String = format!("{} <{}>", &payload.username, &payload.email);
    let message: String = format!("Please copy and paste this link into your browser to confirm your email address: {}/email/verify/{}",smtp_server, hashed_email_token.clone());
    let send_res: bool = match send_email(&from_addr, &to_addr, &email_sub, &message, smtp_server).await {
        Ok(send_res) => send_res,
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if send_res{
        let res: JadeUser = match get_user_by_handle(&payload.username, pool).await {
            Ok(res) => res,
            Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
        };
        Ok(res)
    }
    else {
        let e: String = "Could not send verification email.".to_string();
        Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    }
    

}

/// Attempts to fetch the user with the given handle from the database.
/// If this operation succeeds, an instance of the "JadeUser" structure is
/// returned. If this operation fails, an error is returned. This function
/// is NOT utilized in any API routes.
pub async fn get_user_by_handle(
    username: &String,
    pool: &Pool<Postgres>
) -> Result<JadeUser, JadeErr> {
    let users: Vec<JadeUser> = match sqlx::query_as!(JadeUser, "SELECT * FROM users")
        .fetch_all(pool)
        .await
    {
        Ok(users) => users,
        Err(e) => return Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let mut result: Vec<JadeUser> = Vec::new();
    for user in users {
        if &user.username == username {
            result.push(user);
        }
        else {}
    }
    if result.len() == 1{
        Ok(result[0].clone())
    }
    else{
        let e: String = format!("User \"{}\" does not exist.", &username);
        Err::<JadeUser, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to delete a user given one of their API tokens.
/// If this operation succeeds,  an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn wipe_user(
    payload: &TokenOnlyPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, JadeErr> {
    let token: APIToken = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let _wipe_op: () = match sqlx::query!("DELETE FROM users WHERE username = $1", token.username)
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let status: StatusResponse = StatusResponse{ status: 0 };
    Ok(status)
}

/// Attempts to create a new mood for a user with the given
/// payload. If this operation succeeds, an instance of 
/// the "JadeMood" structure. If this operation fails, an 
/// error is returned.
pub async fn create_new_mood(
    payload: &MoodActionPayload,
    pool: &Pool<Postgres>,
) -> Result<JadeMood, JadeErr> {
    let token: APIToken = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let username: String = token.username;
    let all_moods: Vec<JadeMood> = match sqlx::query_as!(JadeMood, "SELECT * FROM moods")
        .fetch_all(pool)
        .await
    {
        Ok(all_moods) => all_moods,
        Err(e) => return Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
    };
    for mood in all_moods{
        let _update_op: () = match sqlx::query!("UPDATE moods SET is_active = $1 WHERE username = $2", false, mood.username)
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
        };
    }
    if token.can_set_mood{
        let new_mood: JadeMood = JadeMood {
            mood: payload.mood.clone(),
            is_active: true,
            username: username,
            created_at: get_time()
        };
        let _insert_op = match sqlx::query!(
            "INSERT INTO moods (username, is_active, mood, created_at) VALUES ($1, $2, $3, $4)",
            new_mood.username,
            new_mood.is_active,
            new_mood.mood,
            new_mood.created_at,
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
        };
        Ok(new_mood)
    }
    else {
        let e: String = format!("User \"{}\" does not have the correct permissions.", &username);
        Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to delete a mood for a user given 
/// one of their API tokens. If this operation 
/// succeeds, an instance of  the "StatusResponse" 
/// structure is returned with a status code of 0. 
/// If this operation fails, an error is returned 
/// or an instance of the "StatusResponse" structure 
/// with the status code of 1.
pub async fn wipe_mood(
    payload: &MoodActionPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, JadeErr> {
    let token: APIToken = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let _wipe_op: () = match sqlx::query!("DELETE FROM moods WHERE username = $1", token.username)
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let status: StatusResponse = StatusResponse{ status: 0 };
    Ok(status)
}

/// Attempts to create a new API token for a user with
/// the given payload. If this operation succeeds, 
/// an instance of  the "JadeMood" structure. If this 
/// operation fails, an  error is returned.
pub async fn create_new_token(
    payload: &CreateTokenPayload,
    pool: &Pool<Postgres>
) -> Result<APIToken, JadeErr> {
    let user: JadeUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<APIToken, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let is_valid: bool = match verify(&user.pwd,&payload.password){
        Ok(is_valid) => is_valid,
        Err(e) => return Err::<APIToken, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if is_valid {
        let hashed: String = match hash(format!("{}:{}", get_time(), &payload.username), DEFAULT_COST){
            Ok(hashed) => hashed,
            Err(e) => return Err::<APIToken, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let new_token: APIToken = APIToken{
            username: payload.username.clone(),
            token: hashed,
            created_at: get_time(),
            is_active: true,
            can_change_pwd: payload.can_change_pwd,
            can_set_mood: payload.can_set_mood,
            can_delete_user: payload.can_delete_user,
            can_change_email: payload.can_change_email.clone(),
        };
        let _insert_op = match sqlx::query!(
            "INSERT INTO api_tokens (username, token, created_at, is_active, can_change_pwd, can_set_mood, can_delete_user, can_change_email) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            new_token.username,
            new_token.token,
            new_token.created_at,
            new_token.is_active,
            new_token.can_change_pwd,
            new_token.can_set_mood,
            new_token.can_delete_user,
            new_token.can_change_email
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<APIToken, JadeErr>(JadeErr::new(&e.to_string()))
        };
        Ok(new_token)

    }
    else {
        let e: String = format!("Passwords did not match for user \"{}\"!", &payload.username);
        Err::<APIToken, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to delete an API token of a user.
/// If this operation succeeds,  an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn wipe_token(
    payload: &DeleteTokenPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, JadeErr> {
    let token: APIToken = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let user: JadeUser = match get_user_by_handle(&token.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if user.pwd == payload.password{
        let _wipe_op: () = match sqlx::query!("DELETE FROM users WHERE username = $1", token.username)
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let status: StatusResponse = StatusResponse{ status: 0 };
        Ok(status)
    }
    else {
        let e: String = format!("Passwords did not match for user \"{}\"!", &payload.username);
        Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to update the password for a user.
/// If this operation succeeds,  an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn update_user_password(
    payload: &ChangeEntityPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, JadeErr>{
    let token: APIToken = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let user: JadeUser = match get_user_by_handle(&token.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if token.is_active && 
       token.can_change_pwd && 
       token.username == user.username
    {
        let _update_op: () = match sqlx::query!("UPDATE users SET pwd = $1 WHERE username = $2", payload.new_entity, user.username)
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let status: StatusResponse = StatusResponse{ status: 0 };
        Ok(status)
    }
    else {
        let e: String = "Token not active or usernames did not match!".to_string();
        Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to update the mail address for a user.
/// If this operation succeeds, an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn update_user_email(
    payload: &ChangeEntityPayload,
    pool: &Pool<Postgres>,
    smtp_server: &String
) -> Result<StatusResponse, JadeErr>{
    let token: APIToken = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let user: JadeUser = match get_user_by_handle(&token.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if token.is_active && 
       token.can_change_pwd && 
       token.username == user.username 
    {
        let hashed_email: String = match hash(&payload.new_entity, DEFAULT_COST){
            Ok(hashed_email) => hashed_email,
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let _update_op: () = match sqlx::query!("UPDATE users SET email = $1 WHERE username = $2", hashed_email, user.username)
            .execute(pool)
            .await
        {

            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let hashed_email_token = match hash(&format!("{}{}{}", &user.username, &payload.new_entity, get_time()), DEFAULT_COST){
            Ok(hashed) => hashed,
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let _update_token_op: () = match sqlx::query!("UPDATE users SET email_token = $1 WHERE username = $2", hashed_email_token, user.username)
            .execute(pool)
            .await
        {

            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let email_sub: String = format!("Confirm your new email address, {}.", &user.username);
        let from_addr: String = format!("Jade <noreply@{}>", smtp_server);
        let to_addr: String = format!("{} <{}>", &user.username, &payload.new_entity);
        let message: String = format!("Please copy and paste this link into your browser to confirm your email address: {}/email/verify/{}",smtp_server, hashed_email_token.clone());
        let send_res: bool = match send_email(&from_addr, &to_addr, &email_sub, &message, smtp_server).await {
            Ok(send_res) => send_res,
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        if send_res{
            let status: StatusResponse = StatusResponse{ status: 0};
            Ok(status)
        }
        else {
            let e: String = "Could not send verification email.".to_string();
            Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        }
    }
    else {
        let e: String = "Token not active or usernames did not match!".to_string();
        Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to update the name for a user.
/// If this operation succeeds, an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn update_user_display_name(
    payload: &ChangeEntityPayload,
    pool: &Pool<Postgres>,
    smtp_server: &String
) -> Result<StatusResponse, JadeErr>{
    let token: APIToken = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens WHERE token = $1", payload.api_token)
        .fetch_one(pool)
        .await
    {
        Ok(token) => token,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let user: JadeUser = match get_user_by_handle(&token.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if token.is_active && 
       token.can_change_pwd && 
       token.username == user.username 
    {
        let _update_op: () = match sqlx::query!("UPDATE users SET display_name = $1 WHERE username = $2", payload.new_entity, user.username)
            .execute(pool)
            .await
        {

            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        Ok(StatusResponse{status: 0})
    }
    else {
        let e: String = "Token does not have the correct permissions".to_string();
        Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to fetch the mood of a user with the given
/// username. If this operation succeeds, the currently-active
/// instance of the user's mood is returned. If this operation
/// fails, an error is returned.
pub async fn get_user_mood(
    payload: &UsernameOnlyPayload, 
    pool: &Pool<Postgres>
)-> Result<JadeMood, JadeErr>{
    let user: JadeUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let moods: Vec<JadeMood> = match sqlx::query_as!(JadeMood, "SELECT * FROM moods WHERE username = $1", user.username)
        .fetch_all(pool)
        .await
    {
        Ok(moods) => moods,
        Err(e) => return Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let mut result: Vec<JadeMood> = Vec::new();
    for mood in moods {
        if mood.is_active {
            result.push(mood);
        }
        else {}
    }
    if result.len() == 1 {
        Ok(result[0].clone())
    }
    else {
        let e: String = format!("The user \"{}\" either does not exist or has not created any moods.", &user.username);
        Err::<JadeMood, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to retrieve all moods for a user.
/// If this operation is successful, an instance of
/// the "UserMoodsResponse" structure is returned.
/// If this operation fails, an error is returned.
pub async fn get_user_moods(
    payload: &UsernameOnlyPayload, 
    pool: &Pool<Postgres>
) -> Result<UserMoodsResponse, JadeErr>{
    let user: JadeUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<UserMoodsResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let moods: Vec<JadeMood> = match sqlx::query_as!(JadeMood, "SELECT * FROM moods")
        .fetch_all(pool)
        .await
    {
        Ok(moods) => moods,
        Err(e) => return Err::<UserMoodsResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let mut result: Vec<JadeMood> = Vec::new();
    for mood in moods {
        if mood.username == user.username {
            if mood.is_active {}
            else {
                result.push(mood);
            }
        }
        else {}
    }
    let active_mood: JadeMood = match get_user_mood(payload, pool).await {
        Ok(active_mood) => active_mood,
        Err(e) => return Err::<UserMoodsResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(UserMoodsResponse{ active_mood: active_mood, inactive_moods: result})
}

/// Attempts to retrieve all active API tokens for a user.
/// If this operation is successful, a vector of the
/// instances of the "APIToken" structure is returned.
/// If this operation fails, an error is returned.
pub async fn get_user_tokens(
    payload: &UserAPITokensPayload,
    pool: &Pool<Postgres>
) -> Result<Vec<APIToken>, JadeErr>{
    let user: JadeUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<Vec<APIToken>, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if user.pwd == payload.password{
        let tokens: Vec<APIToken>  = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens")
            .fetch_all(pool)
            .await
        {
            Ok(tokens) => tokens,
            Err(e) => return Err::<Vec<APIToken>, JadeErr>(JadeErr::new(&e.to_string()))
        };
        let mut result: Vec<APIToken> = Vec::new();
        for token in tokens {
            if token.username == user.username {
                if token.is_active {}
                else {
                    result.push(token);
                }
            }
            else {}
        }
        Ok(result)
    }
    else {
        let e: String = format!("Passwords do not match for user \"{}\"!", &user.username);
        Err::<Vec<APIToken>, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to get the profile
/// information of a user with 
/// the supplied username. If this
/// operation fails, an error is
/// returned.
pub async fn get_profile_info(
    payload: &UsernameOnlyPayload,
    pool: &Pool<Postgres>
) -> Result<ProfileInfo, JadeErr> {
    let user: JadeUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<ProfileInfo, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let moods: UserMoodsResponse = match get_user_moods(payload, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<ProfileInfo, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let moods_count: usize = moods.inactive_moods.len() + 1;
    let profile: ProfileInfo = ProfileInfo {
        username: user.username,
        avatar_url: user.avatar_url,
        display_name: user.display_name,
        active_mood: moods.active_mood,
        inactive_moods: moods.inactive_moods,
        moods_posted: moods_count
    };
    Ok(profile)
}

/// Attempts to log a user in by creating a session token
/// for a user. If the operation is successful, an instance 
/// of the "JadeUserSessionToken" structure is returned. 
/// If the operation fails, an error is returned.
pub async fn login_user(
    payload: &LoginPayload, 
    pool: &Pool<Postgres>
) -> Result<JadeUserSessionToken, JadeErr> {
    let user: JadeUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<JadeUserSessionToken, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let str_hash: String = format!("{}:{}", &payload.username, get_time());
    let token: String = match hash(str_hash, DEFAULT_COST){
        Ok(token) => token,
        Err(e) => return Err::<JadeUserSessionToken, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let verified: bool = match verify(&payload.password, &user.pwd){
        Ok(verified) => verified,
        Err(e) => return Err::<JadeUserSessionToken, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if verified{
        let new_token: JadeUserSessionToken = JadeUserSessionToken{
            username: user.username,
            token: token
        };
        let _insert_op = match sqlx::query!(
            "INSERT INTO user_session_tokens (username, session_token) VALUES ($1, $2)",
            new_token.username,
            new_token.token
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<JadeUserSessionToken, JadeErr>(JadeErr::new(&e.to_string()))
        };
        Ok(new_token)
    }
    else {
        let e: String = "Username and password do not match.".to_string();
        Err::<JadeUserSessionToken, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to delete a session token for a user.
/// This will only happen when a user logs out.
/// If this operation succeeds,  an instance of 
/// the "StatusResponse" structure is returned 
/// with a status code of 0. If this operation fails, 
/// an error is returned or an instance of the "StatusResponse"
/// structure with the status code of 1.
pub async fn logout_user(
    payload: &JadeUserSessionToken,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, JadeErr> {
    let _wipe_op: () = match sqlx::query!("DELETE FROM user_session_tokens WHERE session_token = $1", payload.token)
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let status: StatusResponse = StatusResponse{ status: 0 };
    Ok(status)
}

/// This function attempts to get
/// an instance of the "APIToken"
/// structure from the database.
/// If the operation fails, an error
/// is returned.
pub async fn get_api_token_object(
    token: &String, 
    pool: &Pool<Postgres>
) -> Result<APIToken, JadeErr>{
    let tokens: Vec<APIToken> = match sqlx::query_as!(APIToken, "SELECT * FROM api_tokens")
        .fetch_all(pool)
        .await
    {
        Ok(tokens) => tokens,
        Err(e) => return Err::<APIToken, JadeErr>(JadeErr::new(&e.to_string()))
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
        let e: String = format!("Token \"{}\" not found.", &token);
        Err::<APIToken, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to follow a user with 
/// the supplied username. If the operation
/// is successful, an instance of the "StatusResponse"
/// structure is returned with a status code of "0".
/// If this operation fails, an error is returned.
pub async fn write_follow_user(
    payload: UserFollowPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, JadeErr> {
    let user_who_is_following: JadeUser = match get_user_from_token(&payload.token, pool).await {
        Ok(user_who_is_following) => user_who_is_following,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let token_obj: APIToken = match get_api_token_object(&payload.token, pool).await {
        Ok(token_obj) => token_obj,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let user_to_follow: JadeUser = match get_user_by_handle(&payload.target_user, pool).await {
        Ok(user_to_follow) => user_to_follow,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    if token_obj.is_active && token_obj.username == user_who_is_following.username {
        let follow_rel: JadeUserFollows = JadeUserFollows{
            follower: user_who_is_following.username,
            followee: user_to_follow.username
        };
        let _insert_op = match sqlx::query!(
            "INSERT INTO user_follows (follower, followee) VALUES ($1, $2)",
            follow_rel.follower,
            follow_rel.followee
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
        };
        Ok(StatusResponse{ status: 0})
    }
    else {
        let e: String = "Token does not have the correct permissions or users do not exist.".to_string();
        Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    }
}

/// Attempts to unfollow a user with 
/// the supplied username. If the operation
/// is successful, an instance of the "StatusResponse"
/// structure is returned with a status code of "0".
/// If this operation fails, an error is returned.
pub async fn write_unfollow_user(
    payload: &UserFollowPayload,
    pool: &Pool<Postgres>
) -> Result<StatusResponse, JadeErr> {
    let user_who_is_following: JadeUser = match get_user_from_token(&payload.token, pool).await {
        Ok(user_who_is_following) => user_who_is_following,
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let _wipe_op: () = match sqlx::query!("DELETE FROM user_follows WHERE follower = $1", user_who_is_following.username)
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<StatusResponse, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let status: StatusResponse = StatusResponse{ status: 0 };
    Ok(status)
}

/// Attempts to retrieve the profile of every user following 
/// the user who requested this information. A vector of profiles
/// is returned if this operation is successful. If this operation
/// fails, an error is returned.
pub async fn get_followers_from_db(
    payload: &UsernameOnlyPayload, 
    pool: &Pool<Postgres>
) -> Result<Vec<ProfileInfo>, JadeErr>{
    let mut result: Vec<ProfileInfo> = Vec::new();
    let follow_rels: Vec<JadeUserFollows> = match sqlx::query_as!(JadeUserFollows, "SELECT * FROM user_follows")
        .fetch_all(pool)
        .await
    {
        Ok(follow_rels) => follow_rels,
        Err(e) => return Err::<Vec<ProfileInfo>, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let user_followed: JadeUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user_followed) => user_followed,
        Err(e) => return Err::<Vec<ProfileInfo>, JadeErr>(JadeErr::new(&e.to_string()))
    };
    for rel in follow_rels {
        if &rel.followee == &user_followed.username {
            let user_to_get: UsernameOnlyPayload = UsernameOnlyPayload{ username: rel.followee};
            let profile_info: ProfileInfo = match get_profile_info(&user_to_get, pool).await {
                Ok(profile_info) => profile_info,
                Err(e) => return Err::<Vec<ProfileInfo>, JadeErr>(JadeErr::new(&e.to_string()))
            };
            result.push(profile_info);
        }
    }
    Ok(result)
}

/// Attempts to retrieve the profile of every user whom
/// the user who requested this information is following. 
/// A vector of profiles is returned if this operation is successful. 
/// If this operation fails, an error is returned.
pub async fn get_following_from_db(
    payload: &UsernameOnlyPayload, 
    pool: &Pool<Postgres>
) -> Result<Vec<ProfileInfo>, JadeErr>{
    let mut result: Vec<ProfileInfo> = Vec::new();
    let follow_rels: Vec<JadeUserFollows> = match sqlx::query_as!(JadeUserFollows, "SELECT * FROM user_follows")
        .fetch_all(pool)
        .await
    {
        Ok(follow_rels) => follow_rels,
        Err(e) => return Err::<Vec<ProfileInfo>, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let user_following: JadeUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user_followed) => user_followed,
        Err(e) => return Err::<Vec<ProfileInfo>, JadeErr>(JadeErr::new(&e.to_string()))
    };
    for rel in follow_rels {
        if &rel.follower == &user_following.username {
            let user_to_get: UsernameOnlyPayload = UsernameOnlyPayload{ username: rel.follower};
            let profile_info: ProfileInfo = match get_profile_info(&user_to_get, pool).await {
                Ok(profile_info) => profile_info,
                Err(e) => return Err::<Vec<ProfileInfo>, JadeErr>(JadeErr::new(&e.to_string()))
            };
            result.push(profile_info);
        }
    }
    Ok(result)
}

/// Attempts to retrieve the all the 
/// active moods of users a user is following
/// A vector of moods is returned. If this
/// operation fails, an error is returned.
pub async fn get_time_line(
    payload: &UsernameOnlyPayload, 
    pool: &Pool<Postgres>
) -> Result<Vec<JadeMood>, JadeErr>{
    let following: Vec<ProfileInfo> = match get_following_from_db(payload, pool).await {
        Ok(following) => following,
        Err(e) => return Err::<Vec<JadeMood>, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let mut result: Vec<JadeMood> = Vec::new();
    for person in following {
        result.push(person.active_mood);
    }
    Ok(result)
}