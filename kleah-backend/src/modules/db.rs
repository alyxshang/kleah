/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// representing a pool of
/// database connections.
use sqlx::Pool;

/// Importing the macro
/// to execute SQL queries 
/// that return nothing.
use sqlx::query;

/// Importing the function
/// from the "bcrypt" library
/// to hash and salt a piece of
/// text.
use bcrypt::hash;

/// Importing the macro
/// to execute SQL queries 
/// that return an object.
use sqlx::query_as;

/// Importing the structure
/// for catching and handling
/// errors.
use super::err::KleahErr;

/// Importing the entity
/// that represents the default
/// computational cost for hashing
/// a string with the BCrypt algorithm.
use bcrypt::DEFAULT_COST;

/// Importing the structure
/// that stores information
/// about a user's RSA keypair.
use super::units::KeyPair;

/// Importing the function
/// to generate a string
/// containing the current
/// time in the RFC2282 format.
use super::utils::rfc2282;

/// Importing the structure that
/// represents a connection to a 
/// PostgreSQL database.
use sqlx::postgres::Postgres;

/// Importing the data structure
/// modelling data about a Kleah user
/// in the database.
use super::models::KleahUser;

/// Importing the data structure
/// modelling data about an ActivityPub
/// actor on a Kleah instance in the 
/// database.
use super::models::KleahActor;

/// Importing the function to
/// generate a SHA-256 hash
/// as a string of the given
/// string.
use super::utils::hash_string;

/// Importing the data structure
/// modelling a user's API token
/// in the database.
use super::models::UserAPIToken;

/// Importing the function for generating
/// an RSA keypair for a user.
use super::utils::generate_keypair;

/// Importing the data structure
/// modelling data about the current
/// Kleah instance in the database.
use super::models::InstanceInformation;

/// A function to create a new record
/// for a new Kleah user in the database.
/// If the operation is successful, an 
/// instance of the `KleahUser` structure
/// is returned. If the operation fails,
/// an error is returned.
pub async fn create_new_user(
    name: &str,
    password: &str,
    username: &str,
    email_addr: &str,
    description: &str,
    is_admin: &bool,
    pool: &Pool<Postgres>
) -> Result<KleahUser, KleahErr>{
    let hashed_pwd: String = match hash(
        password, 
        DEFAULT_COST
    ){
        Ok(hashed_pwd) => hashed_pwd,
        Err(e) => return Err::<KleahUser, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let hashed_email: String = match hash(
        email_addr, 
        DEFAULT_COST
    ){
        Ok(hashed_email) => hashed_email,
        Err(e) => return Err::<KleahUser, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let pair: KeyPair = match generate_keypair(){
        Ok(pair) => pair,
        Err(e) => return Err::<KleahUser, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let obj: KleahUser = KleahUser{
        name: name.to_string(),
        username: username.to_string(),
        password: hashed_pwd,
        email_addr: hashed_email,
        public_key: pair.public_key,
        description: description.to_string(),
        private_key: pair.private_key,
        is_admin: *is_admin
    };
    let _insert_op = match query!(
        "INSERT INTO users (name, username, password, email_addr, public_key, description, private_key, is_admin) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        obj.name,
        obj.username,
        obj.password,
        obj.email_addr,
        obj.public_key,
        obj.description,
        obj.private_key,
        obj.is_admin
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<KleahUser, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let fetched: KleahUser = match get_user_by_id(
        &obj.username,
        pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<KleahUser, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(fetched)
}

/// A function to create a new record
/// for a new Kleah ActivityPub actor 
/// in the database. If the operation 
/// is successful, an instance of the 
/// `KleahActor` structure is returned. 
/// If the operation fails,
/// an error is returned.
pub async fn create_new_actor(
    name: &str,
    host: &str,
    username: &str,
    actor_type: &str,
    description: &str,
    liked_endpoint: &str,
    inbox_endpoint: &str,
    outbox_endpoint: &str,
    following_endpoint: &str,
    followers_endpoint: &str,
    public_key_endpoint: &str,
    pool: &Pool<Postgres>
) -> Result<KleahActor, KleahErr>{
    let obj: KleahActor = KleahActor {
        name: name.to_string(), 
        actor_type: actor_type.to_string(), 
        host: host.to_string(), 
        liked: liked_endpoint.to_string(), 
        inbox: inbox_endpoint.to_string(), 
        outbox: outbox_endpoint.to_string(), 
        following: following_endpoint.to_string(), 
        followers: followers_endpoint.to_string(), 
        username: username.to_string(), 
        description: description.to_string(), 
        public_key: public_key_endpoint.to_string()
    };
    let _insert_op = match query!(
        "INSERT INTO actors (name, actor_type, host, liked, inbox, outbox, following, followers, username, description, public_key) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        obj.name,
        obj.actor_type,
        obj.host,
        obj.liked,
        obj.inbox,
        obj.outbox,
        obj.following,
        obj.followers,
        obj.username,
        obj.description,
        obj.public_key
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<KleahActor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let fetched: KleahActor = match get_actor_by_id(
        &obj.username,
        &obj.host,
        pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<KleahActor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(fetched)

}

/// A function that attempts to retrieve a record
/// about a Kleah ActivityPub actor given the 
/// user's username and host. If the operation is 
/// successful, an instance of the `KleahUser` 
/// structure is returned. If the operation 
/// fails, an error is returned.
pub async fn get_actor_by_id(
    username: &str,
    host: &str,
    pool: &Pool<Postgres>
) -> Result<KleahActor, KleahErr>{
    let object: KleahActor = match query_as!(
        KleahActor,
        "SELECT * FROM actors WHERE username = $1 AND host = $2",
        username,
        host
    )
        .fetch_one(pool)
        .await 
    {
        Ok(object) => object,
        Err(e) => return Err::<KleahActor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(object)

}

/// A function that attempts to retrieve a record
/// about a Kleah user given the user's username.
/// If the operation is successful, an instance of
/// the `KleahUser` structure is returned. If the
/// operation fails, an error is returned.
pub async fn get_user_by_id(
    username: &str,
    pool: &Pool<Postgres>
) -> Result<KleahUser, KleahErr>{
    let object: KleahUser = match query_as!(
        KleahUser,
        "SELECT * FROM users WHERE username = $1",
        username
    )
        .fetch_one(pool)
        .await 
    {
        Ok(object) => object,
        Err(e) => return Err::<KleahUser, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(object)
}

/// A function to create a new record
/// containing data about the current
/// Kleah instance in the database. 
/// If the operation is successful, 
/// an instance of the `InstanceInformation` 
/// structure is returned. 
/// If the operation fails,
/// an error is returned.
pub async fn create_instance_info(
    host: &str,
    uses_invites: &bool,
    pool: &Pool<Postgres>
) -> Result<InstanceInformation, KleahErr>{
    let obj: InstanceInformation = InstanceInformation{ 
        host: host.to_string(),
        uses_invites: *uses_invites
    };
    let _insert_op = match query!(
        "INSERT INTO instance_information (uses_invites, host) VALUES ($1, $2)",
        obj.uses_invites,
        obj.host
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<InstanceInformation, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let fetched: InstanceInformation = match get_instance_info(
        &pool
    ).await {
        Ok(fetched) => fetched,
        Err(e) => return Err::<InstanceInformation, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    if fetched.host == obj.host{
        Ok(fetched)
    }
    else {
        Err::<InstanceInformation, KleahErr>(
            KleahErr::new("Error fetching instance information.")
        )
    }
}

/// A function that attempts to retrieve a record
/// about a Kleah instance. If the operation is 
/// successful, an instance of the `InstanceInformation` 
/// structure is returned. If the operation 
/// fails, an error is returned.
pub async fn get_instance_info(
    pool: &Pool<Postgres>
) -> Result<InstanceInformation, KleahErr>{
    let object: InstanceInformation = match query_as!(
        InstanceInformation,
        "SELECT * FROM instance_information",
    )
        .fetch_one(pool)
        .await 
    {
        Ok(object) => object,
        Err(e) => return Err::<InstanceInformation, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(object)
}

/// This function attempts to create a new API token
/// for a user and store it in the database. If the 
/// operation is successful, an instance of the 
/// `UserAPIToken` is returned. If the operation fails,
/// an error is returned.
pub async fn create_api_token(
    username: &str,
    pool: &Pool<Postgres>
) -> Result<UserAPIToken, KleahErr>{
    let fetched: KleahUser = match get_user_by_id(
        username,
        pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<UserAPIToken, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let token_str: String = hash_string(
        &format!(
            "{}{}{}", 
            fetched.username,
            fetched.name,
            rfc2282()
        )
    );
    let obj: UserAPIToken = UserAPIToken{ 
        username: fetched.username, 
        token: token_str
    };
    let _insert_op = match query!(
        "INSERT INTO user_api_tokens (username, token) VALUES ($1, $2)",
        obj.username,
        obj.token
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<UserAPIToken, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let fetched_token: UserAPIToken = match get_token_by_token(
        &obj.token,
        pool
    ).await {
        Ok(fetched_token) => fetched_token,
        Err(e) => return Err::<UserAPIToken, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(fetched_token)
}

/// A function that attempts to retrieve a record
/// about a Kleah user's API token given the user's
/// API token string. If the operation is successful, 
/// an instance of the `UserAPIToken` structure is 
/// returned. If the operation fails, an error is 
/// returned.
pub async fn get_token_by_token(
    token: &str,
    pool: &Pool<Postgres>
) -> Result<UserAPIToken, KleahErr>{
    let object: UserAPIToken = match query_as!(
        UserAPIToken,
        "SELECT * FROM user_api_tokens WHERE token = $1",
        token
    )
        .fetch_one(pool)
        .await 
    {
        Ok(object) => object,
        Err(e) => return Err::<UserAPIToken, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(object)
}

/// This function checks
/// all records in the database
/// for the record of a user with
/// the given username. The existence
/// of this record is reflected by
/// the boolean value returned.
pub async fn user_exists(
    username: &str,
    pool: &Pool<Postgres>
) -> bool {
    get_user_by_id(username, pool)
        .await
        .is_ok()
}

/// Attempts to retrieve the record of a user
/// from the database given one of their API
/// tokens. If the operation is successful,
/// an instance of the `KleahUser` structure
/// is returned. If the operation fails, an 
/// error is returned.
pub async fn get_user_by_token(
    token: &str,
    pool: &Pool<Postgres>
) -> Result<KleahUser, KleahErr>{
    let fetched_token: UserAPIToken = match get_token_by_token(
        token,
        pool
    ).await {
        Ok(fetched_token) => fetched_token,
        Err(e) => return Err::<KleahUser, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let fetched: KleahUser = match get_user_by_id(
        &fetched_token.username,
        pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<KleahUser, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(fetched)
}

/// Attempts to update the password column of
/// a user's record in the database. If the
/// operation is successful, nothing is
/// returned. If the operation fails, an error
/// is returned.
pub async fn update_password(
    username: &str,
    new_password: &str,
    pool: &Pool<Postgres>
) -> Result<(), KleahErr>{
    let new_hashed_pwd: String = match hash(new_password, DEFAULT_COST){
        Ok(new_hashed_pwd) => new_hashed_pwd,
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let fetched_user: KleahUser = match get_user_by_id(
        &username,
        pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let update_op: () = match query!(
        "UPDATE users SET password = $1 WHERE username = $2",
        new_hashed_pwd,
        fetched_user.username
    )
        .execute(pool)
        .await
    {
        Ok(_f) => {},
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(update_op)
}

/// Attempts to update the email column of
/// a user's record in the database. If the
/// operation is successful, nothing is
/// returned. If the operation fails, an error
/// is returned.
pub async fn update_email(
    username: &str,
    new_email: &str,
    pool: &Pool<Postgres>
) -> Result<(), KleahErr>{
    let new_hashed_email: String = match hash(new_email, DEFAULT_COST){
        Ok(new_hashed_email) => new_hashed_email,
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let fetched_user: KleahUser = match get_user_by_id(
        &username,
        pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let update_op: () = match query!(
        "UPDATE users SET email_addr = $1 WHERE username = $2",
        new_hashed_email,
        fetched_user.username
    )
        .execute(pool)
        .await
    {
        Ok(_f) => {},
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(update_op)
}

/// Attempts to update the description column of
/// a user's record and the corresponding actor's 
/// record in the database. If the
/// operation is successful, nothing is
/// returned. If the operation fails, an error
/// is returned.
pub async fn update_description(
    username: &str,
    new_description: &str,
    pool: &Pool<Postgres>
) -> Result<(), KleahErr>{
    let fetched_user: KleahUser = match get_user_by_id(
        &username,
        pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let instance_info: InstanceInformation = match get_instance_info(
        pool
    ).await {
        Ok(instance_info) => instance_info,
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let fetched_actor: KleahActor = match get_actor_by_id(
        &fetched_user.username,
        &instance_info.host,
        pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let _update_op: () = match query!(
        "UPDATE users SET description = $1 WHERE username = $2",
        new_description,
        fetched_user.username
    )
        .execute(pool)
        .await
    {
        Ok(_f) => {},
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let update_op: () = match query!(
        "UPDATE actors SET description = $1 WHERE username = $2",
        new_description,
        fetched_actor.username
    )
        .execute(pool)
        .await
    {
        Ok(_f) => {},
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(update_op)
}

/// Attempts to update the name column of
/// a user's record and the corresponding actor's 
/// record in the database. If the
/// operation is successful, nothing is
/// returned. If the operation fails, an error
/// is returned.
pub async fn update_name(
    username: &str,
    new_name: &str,
    pool: &Pool<Postgres>
) -> Result<(), KleahErr>{
    let fetched_user: KleahUser = match get_user_by_id(
        &username,
        pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let instance_info: InstanceInformation = match get_instance_info(
        pool
    ).await{
        Ok(instance_info) => instance_info,
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let fetched_actor: KleahActor = match get_actor_by_id(
        &fetched_user.username,
        &instance_info.host,
        pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };

    let _update_op: () = match query!(
        "UPDATE users SET name = $1 WHERE username = $2",
        new_name,
        fetched_user.username
    )
        .execute(pool)
        .await
    {
        Ok(_f) => {},
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let update_op: () = match query!(
        "UPDATE actors SET name = $1 WHERE username = $2",
        new_name,
        fetched_actor.username
    )
        .execute(pool)
        .await
    {
        Ok(_f) => {},
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(update_op)
}

/// Attempts to delete a record of an
/// API token from the database. If the
/// operation is successful, nothing is
/// returned. If the operation fails,
/// an error is returned.
pub async fn destroy_token(
    username: &str,
    token: &str,
    pool: &Pool<Postgres>
) -> Result<(), KleahErr>{
    let fetched_token: UserAPIToken = match get_token_by_token(
        token,
        pool
    ).await {
        Ok(fetched_token) => fetched_token,
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let fetched_user: KleahUser = match get_user_by_id(
        &username,
        pool
    ).await {
        Ok(user) => user,
        Err(e) => return Err::<(), KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    if fetched_token.username == fetched_user.username {
        let del_op: () = match query!(
            "DELETE FROM user_api_tokens WHERE token = $1 AND username = $2",
            fetched_token.token,
            fetched_user.username
        )
            .execute(pool)
            .await 
        {
            Ok(_f) => {},
            Err(e) => return Err::<(), KleahErr>(
                KleahErr::new(&e.to_string())
            )
        };
        Ok(del_op)
    }
    else {
        Err::<(), KleahErr>(
            KleahErr::new("User and token owner do not match.")
        )
    }
}
