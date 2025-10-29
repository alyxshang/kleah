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
        private_key: pair.private_key
    };
    let _insert_op = match query!(
        "INSERT INTO users (name, username, password, email_addr, public_key, description, private_key) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        obj.name,
        obj.username,
        obj.password,
        obj.email_addr,
        obj.public_key,
        obj.description,
        obj.private_key,
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
    pool: &Pool<Postgres>
) -> Result<InstanceInformation, KleahErr>{
    let obj: InstanceInformation = InstanceInformation{ 
        host: host.to_string() 
    };
    let _insert_op = match query!(
        "INSERT INTO instance_information (host) VALUES ($1)",
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
