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

/// Importing the function
/// to hash a string using
/// the "BCrypt" algorithm.
use bcrypt::hash;

/// Importing the function
/// to verify a hashed string.
use bcrypt::verify;

/// Importing the "query_as"
/// macro to execute queries
/// that return something.
use sqlx::query_as;

/// Importing the default
/// settings for salting
/// and hashing.
use bcrypt::DEFAULT_COST;

/// Importing the data structure
/// to catch and handle errors.
use super::err::KleahErr;

/// Importing the data structure
/// for creating a public actor.
use super::models::Actor;

/// Importing the "TimeNow"
/// structure to get the current
/// time.
use super::utils::TimeNow;

/// Importing the data structure
/// to save a keypair.
use super::units::KeyPair;

/// Importing the "Postgres"
/// structure for explicit 
/// typing.
use sqlx::postgres::Postgres;

/// Importing the model for
/// storing instance information
/// in the database.
use super::models::InstanceInfo;

/// Importing the data structure
/// for creating a private actor,
/// i.e. a user with confidential
/// information.
use super::models::PrivateActor;

/// Importing the function to
/// generate a public and private
/// keypair for a user.
use super::utils::generate_keypair;

/// A function to create a private actor,
/// an actor containing a user's confidential
/// information and a public actor, containing
/// info freely available. If the operation is
/// successful, an instance of a public actor is
/// returned. If the operation fails, an error
/// is returned.
pub async fn create_user(
    username: &String,
    password: &String,
    is_admin: &bool,
    email_addr: &String,
    display_name: &String,
    user_type: &String,
    pool: &Pool<Postgres>
) -> Result<Actor, KleahErr> {
    let hashed_pwd: String = match hash(
            password, 
            DEFAULT_COST
    ){
        Ok(hashed_pwd) => hashed_pwd,
        Err(e) => return Err::<Actor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let keys: KeyPair = match generate_keypair(){
        Ok(keys) => keys,
        Err(e) => return Err::<Actor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let private_actor: PrivateActor = PrivateActor{
        username: username.to_string(),
        email: email_addr.to_string(),
        verified: false,
        privileged: *is_admin,
        private_key: keys.private,
        public_key: keys.public,
        user_password: hashed_pwd
    };
    let inst: InstanceInfo = match get_instance_info(pool).await {
        Ok(saved) => saved,
        Err(e) => return Err::<Actor, KleahErr>(
                KleahErr::new(&e.to_string())
        )
    };
    let actor: Actor = Actor {
        user_id: username.to_string(),
        host: inst.instance_host,
        user_type: user_type.to_string(),
        preferred_username: username.to_string(),
        display_name: display_name.to_string(),
        summary: "".to_string(),
        manually_approves_followers: false,
        discoverable: true,
        indexable: true,
        published: TimeNow::new().to_string(),
        memorial: false,
    };
    let _pa_insert_op: () = match query!(
        "INSERT INTO private_actors (username, email, verified, privileged, private_key, public_key, user_password) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        private_actor.username,
        private_actor.email,
        private_actor.verified,
        private_actor.privileged,
        private_actor.private_key,
        private_actor.public_key,
        private_actor.user_password
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<Actor, KleahErr>(
                KleahErr::new(&e.to_string())
            )
    };
    let _a_insert_op: () = match query!(
        "INSERT INTO actors (user_id, host, user_type, preferred_username, display_name, summary, manually_approves_followers, discoverable, indexable, published, memorial) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        actor.user_id,
        actor.host,
        actor.user_type,
        actor.preferred_username,
        actor.display_name,
        actor.summary,
        actor.manually_approves_followers,
        actor.discoverable,
        actor.indexable,
        actor.published,
        actor.memorial
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<Actor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let saved_pa: PrivateActor = match  get_private_actor_by_name(
        username, 
        pool
    ).await {
        Ok(saved_pa) => saved_pa,
        Err(e) => return Err::<Actor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let saved_a: Actor = match get_actor_by_name(
        username, 
        pool
    ).await{
        Ok(saved_a) => saved_a,
        Err(e) => return Err::<Actor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    if saved_a.user_id == saved_pa.username{
        Ok(saved_a)
    }
    else {
        Err::<Actor, KleahErr>(
            KleahErr::new("Private and public actor could not be created.")
        )
    }
}

/// A function to retrieve an instance
/// of a private actor by their username.
/// If the operation fails, an error is returned.
pub async fn get_private_actor_by_name(
    name: &String,
    pool: &Pool<Postgres>
) -> Result<PrivateActor, KleahErr> {
    let pa_obj: PrivateActor = match query_as!(
        PrivateActor,
        "SELECT * FROM private_actors WHERE username = $1",
        name
    ).fetch_one(pool).await {
        Ok(pa_obj) => pa_obj,
        Err(e) => return Err::<PrivateActor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(pa_obj)
}

/// A function to retrieve an instance
/// of a public actor by their username.
/// If the operation fails, an error is returned.
pub async fn get_actor_by_name(
    name: &String,
    pool: &Pool<Postgres>
) -> Result<Actor, KleahErr> {
    let a_obj: Actor = match query_as!(
        Actor,
        "SELECT * FROM actors WHERE user_id = $1",
        name
    ).fetch_one(pool).await {
        Ok(a_obj) => a_obj,
        Err(e) => return Err::<Actor, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(a_obj)
}

/// A function to retrieve an instance
/// of a private actor by their username.
/// If the actor's user ID matches the supplied
/// one, a boolean `true` is returned. If the 
/// operation fails, an error is returned.
pub async fn private_actor_exists(
    name: &String,
    pool: &Pool<Postgres>
) -> Result<bool, KleahErr> {
    let pa_obj: PrivateActor = match query_as!(
        PrivateActor,
        "SELECT * FROM private_actors WHERE username = $1",
        name
    ).fetch_one(pool).await {
        Ok(pa_obj) => pa_obj,
        Err(e) => return Err::<bool, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let mut result = false;
    if &pa_obj.username == name{
        result = true;
    }
    Ok(result)
}

/// A function to retrieve an instance
/// of a private actor by their username.
/// If the actor's user ID matches the supplied
/// one, a boolean `true` is returned. If the 
/// operation fails, an error is returned.
pub async fn actor_exists(
    name: &String,
    pool: &Pool<Postgres>
) -> Result<bool, KleahErr> {
    let a_obj: Actor = match query_as!(
        Actor,
        "SELECT * FROM actors WHERE user_id = $1",
        name
    ).fetch_one(pool).await {
        Ok(a_obj) => a_obj,
        Err(e) => return Err::<bool, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let mut result = false;
    if &a_obj.user_id == name{
        result = true;
    }
    Ok(result)
}

/// A function to save information
/// about a Kleah instance to the
/// database. This is done when the
/// server is created for the first time.
pub async fn create_instance_info(
    inst_name: &String,
    inst_host: &String,
    inst_smtp: &String,
    inst_pass: &String,
    inst_admin: &String,
    inst_description: &String,
    pool: &Pool<Postgres>
) ->Result<InstanceInfo, KleahErr> {
    let inst: InstanceInfo = InstanceInfo{
        instance_name: inst_name.to_string(),
        instance_host: inst_host.to_string(),
        instance_smtp: inst_smtp.to_string(),
        instance_pass: inst_pass.to_string(),
        instance_admin: inst_admin.to_string(),
        instance_description: inst_description.to_string()
    };
    let exists: bool = match private_actor_exists(inst_admin, pool).await {
        Ok(exists) => exists,
        Err(e) => return Err::<InstanceInfo, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    if exists{
        let _insert_op: () = match query!(
            "INSERT INTO instance_info (instance_name, instance_host, instance_smtp, instance_pass, instance_admin, instance_description) VALUES ($1, $2, $3, $4, $5, $6)",
            inst.instance_name,
            inst.instance_host,
            inst.instance_smtp,
            inst.instance_pass,
            inst.instance_admin,
            inst.instance_description,
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<InstanceInfo, KleahErr>(
                KleahErr::new(&e.to_string())
            )
        };
        let saved: InstanceInfo = match get_instance_info(pool).await {
            Ok(saved) => saved,
            Err(e) => return Err::<InstanceInfo, KleahErr>(
                KleahErr::new(&e.to_string())
            )
        };
        if saved.instance_name != inst.instance_name{
            Err::<InstanceInfo, KleahErr>(
                KleahErr::new("Failed to write instance info.")
            )
        }
        else {
            Ok(saved)
        }
    }
    else {
        Err::<InstanceInfo, KleahErr>(
            KleahErr::new("An administrator must exist.")
        )
    }
}

/// A general function to retrieve
/// information on the current Kleah
/// instance.
pub async fn get_instance_info(
    pool: &Pool<Postgres>
) -> Result<InstanceInfo, KleahErr>{
    let inst_obj: InstanceInfo= match query_as!(
        InstanceInfo,
        "SELECT * FROM instance_info"
    ).fetch_one(pool).await {
        Ok(inst_obj) => inst_obj,
        Err(e) => return Err::<InstanceInfo, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(inst_obj)
}
