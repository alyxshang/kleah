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

/// Importing the data
/// structure for crudding
/// notes a user has created.
use super::models::Note;

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

/// Importing the structure
/// for modelling an activity
/// in the database.
use super::models::UserAct;

/// Importing the "Postgres"
/// structure for explicit 
/// typing.
use sqlx::postgres::Postgres;

/// Importing the function to
/// generate a SHA-256 of a
/// string.
use super::utils::hash_string;

/// Importing the "UserAPIToken"
/// structure to create a new API
/// token for a user.
use super::models::UserAPIToken;

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

pub async fn create_api_token(
    username: &String,
    password: &String,
    pool: &Pool<Postgres>
) -> Result<UserAPIToken, KleahErr> {
    let pa: PrivateActor = match get_private_actor_by_name(
        &username,
        pool
    ).await {
        Ok(pa) => pa,
        Err(e) => return Err::<UserAPIToken, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let verified: bool = match verify(
        password, 
        &pa.user_password
    ){
        Ok(verified) => verified,
        Err(e) => return Err::<UserAPIToken, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    if verified{
        let token_base: String = format!(
            "{}{}",
            TimeNow::new().to_string(),
            username
        );
        let token_str: String = hash_string(&token_base);
        let token: UserAPIToken = UserAPIToken{
            token: token_str.clone(),
            user_id: username.clone()
        };
        let _insert_op: () = match query!(
             "INSERT INTO user_api_tokens (token, user_id) VALUES ($1, $2)",
             token.token,
             token.user_id
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<UserAPIToken, KleahErr>(
                KleahErr::new(&e.to_string())
            )
        };
        let inserted: UserAPIToken = match get_token_by_token(
            &token_str, 
            pool
        ).await {
            Ok(inserted) => inserted,
            Err(e) => return Err::<UserAPIToken, KleahErr>(
                KleahErr::new(&e.to_string())
            )
        };
        Ok(inserted)
    }
    else {
        Err::<UserAPIToken, KleahErr>(
            KleahErr::new("Password verification failed.")
        )
    }
}

/// A function to retrieve an instance
/// of a private actor's API token given 
/// their username. If the operation fails, 
/// an error is returned.
pub async fn get_token_by_token(
    name: &String,
    pool: &Pool<Postgres>
) -> Result<UserAPIToken, KleahErr> {
    let token_obj: UserAPIToken = match query_as!(
        UserAPIToken,
        "SELECT * FROM user_api_tokens WHERE token = $1",
        name
    ).fetch_one(pool).await {
        Ok(token_obj) => token_obj,
        Err(e) => return Err::<UserAPIToken, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(token_obj)
}

/// A function to retrieve
/// a note given the note's
/// ID. If the operation is
/// successful, an instance of
/// the `Note` data structure
/// is returned. If the operation
/// fails, an error is returned.
pub async fn get_note_by_id(
    note_id: &String,
    pool: &Pool<Postgres>
) -> Result<Note, KleahErr>{
    let note_obj: Note = match query_as!(
        Note,
        "SELECT * FROM notes WHERE note_id = $1",
        note_id
    ).fetch_one(pool).await {
        Ok(note_obj) => note_obj,
        Err(e) => return Err::<Note, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(note_obj)
}

pub async fn create_note(
    username: &String,
    is_reply: bool,
    content: &String,
    sensitive: &bool,
    reply_to: &Option<String>,
    pool: &Pool<Postgres>
) -> Result<Note, KleahErr>{
    let hash_source: String = format!(
        "{}{}{}",
        username,
        TimeNow::new().to_string(),
        content
    );
    let note_id: String = hash_string(&hash_source);
    let reply_to_id: String;
    match reply_to{
        Some(id) => reply_to_id = id.to_string(),
        None => reply_to_id = "".to_string()
    };
    let new_note: Note = Note {
        note_id: note_id.clone(),
        author: username.to_string(),
        published: TimeNow::new().to_string(),
        content: content.to_string(),
        like_count: 0,
        boost_count: 0,
        share_count: 0,
        is_reply: is_reply,
        reply_to: reply_to_id,
        sensitive: *sensitive
    };
    let _a_insert_op: () = match query!(
        "INSERT INTO notes (note_id, author, published, content, like_count, boost_count, share_count, is_reply, reply_to, sensitive) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        new_note.note_id,
        new_note.author,
        new_note.published,
        new_note.content,
        new_note.like_count,
        new_note.boost_count,
        new_note.share_count,
        new_note.is_reply,
        new_note.reply_to,
        new_note.sensitive
    )
        .execute(pool)
        .await
    {
        Ok(_feedback) => {},
        Err(e) => return Err::<Note, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let saved: Note = match get_note_by_id(
        &note_id.clone(), 
        pool
    ).await {
        Ok(saved) => saved,
        Err(e) => return Err::<Note, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(saved)
}

pub async fn get_user_act_by_id(
    activity_id: &String,
    pool: &Pool<Postgres>
) -> Result<UserAct, KleahErr>{
    let act_obj: UserAct = match query_as!(
        UserAct,
        "SELECT * FROM user_acts WHERE activity_id = $1",
        activity_id
    ).fetch_one(pool).await {
        Ok(act_obj) => act_obj,
        Err(e) => return Err::<UserAct, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(act_obj)
}

pub async fn create_activity(
    activity_type: &String,
    activity_author: &String,
    subject: &String,
    pool: &Pool<Postgres>
) -> Result<UserAct, KleahErr>{
    let id: String = hash_string(
        &format!(
            "{}{}{}",
            activity_author,
            activity_type,
            TimeNow::new().to_string()
        )
    );
    if activity_type == "Create" ||
        activity_type == "Follow" ||
        activity_type == "Delete" ||
        activity_type == "Like" ||
        activity_type == "Announce"
    {
        let new_activity: UserAct = UserAct{
            activity_id: id.clone(),
            activity_type: activity_type.clone().to_string(),
            activity_author: activity_author.to_string(),
            published_at: TimeNow::new().to_string(),
            object_id: subject.to_string()
        };
        let _a_insert_op: () = match query!(
            "INSERT INTO user_acts (activity_id, activity_type,activity_author, published_at, object_id) VALUES ($1, $2, $3, $4, $5)",
            new_activity.activity_id,
            new_activity.activity_type,
            new_activity.activity_author,
            new_activity.published_at,
            new_activity.object_id
        )
            .execute(pool)
            .await
        {
            Ok(_feedback) => {},
            Err(e) => return Err::<UserAct, KleahErr>(
                KleahErr::new(&e.to_string())
            )
        };
        let saved: UserAct = match get_user_act_by_id(
            &id, 
            pool
        ).await {
            Ok(saved) => saved,
            Err(e) => return Err::<UserAct, KleahErr>(
                KleahErr::new(&e.to_string())
            )
        };
        Ok(saved)
    }
    else {
        Err::<UserAct, KleahErr>(
            KleahErr::new("Wrong activity type supplied.")
        )
    }
}
