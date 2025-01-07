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

/// Importing this crate's
/// structure to catch and
/// handle errors.
use crate::KleahErr;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::Postgres;

/// Importing the "Charm"
/// structure to work with charms
/// and explicitly declare
/// them.
use crate::models::Charm;

/// Importing the "KleahUser"
/// structure to work with users
/// and explicitly declare
/// them.
use crate::models::KleahUser;

/// Importing the structure
/// to store files for a user.
use crate::models::KleahUserFile;

/// Importing the structure that
/// returns a user profile.
use crate::responses::UserProfile;

/// Importing the structure
/// that models detailed info
/// on a charm.
use crate::responses::CharmDetail;

/// Importing the structure
/// that models detailed info
/// on a user's timeline.
use crate::responses::UserTimeline;

/// Importing the structure that models
/// a follower relationship between two
/// Kleah users.
use crate::models::KleahUserFollows;

/// Importing the function to show a 
/// sanitized version of a charm given
/// its ID.
use crate::charms::show_charm_detail;

/// Importing the payload for controlling
/// what a user looking at a another user's
/// profile can see.
use crate::payloads::PublicUserViewPayload;

/// Importing the function to retrieve
/// a user by their handle.
use crate::rw_utils::get_user_by_handle;

/// Importing the structure to submit
/// a payload for obtaining detailed
/// info for a charm.
use crate::payloads::CharmDetailPayload;

/// Importing the response for showing 
/// a user's files.
use crate::responses::UserFilesTimeline;

/// Attempts to retrieve a user's timeline for public
/// viewing. If this operation succeeds, an instance of the
/// "UserTimeline" structure is returned. If this operation
/// fails, however, an error is returned.
pub async fn show_user_timeline_public(
    payload: &PublicUserViewPayload,
    pool: &Pool<Postgres>
) -> Result<UserTimeline, KleahErr> {
    let target_user: KleahUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<UserTimeline, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if target_user.is_private {
        let e: &str = "You are not allowed to view this content.";
        return Err::<UserTimeline, KleahErr>(KleahErr::new(&e.to_string()))
    }
    else {
        let user_charms: Vec<Charm> = match query_as!(Charm, "SELECT * FROM charms WHERE user_id = $1", target_user.user_id)
            .fetch_all(pool)
            .await
        {
            Ok(user_charms) => user_charms,
            Err(e) => return Err::<UserTimeline, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let mut result: Vec<CharmDetail> = Vec::new();
        for charm in user_charms {
            let charm_id: String = charm.charm_id;
            let charm_detail: CharmDetail = match show_charm_detail(&CharmDetailPayload{charm_id: charm_id}, pool).await {
                Ok(charm_detail) => charm_detail,
                Err(e) => return Err::<UserTimeline, KleahErr>(KleahErr::new(&e.to_string()))
            };
            result.push(charm_detail)
        }
        Ok(UserTimeline{charms: result})
    }
}

/// Attempts to collect all profile information on a user
/// for public viewing. If this operation is successful, 
/// an instance of the "UserProfile" structure is returned. 
/// If this operation is not successful, an error is returned.
pub async fn assemble_profile_public(
    payload: &PublicUserViewPayload,
    pool: &Pool<Postgres>
) -> Result<UserProfile, KleahErr> {
    let target_user: KleahUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<UserProfile, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if target_user.is_private {
        let e: &str = "You are not allowed to view this content.";
        return Err::<UserProfile, KleahErr>(KleahErr::new(&e.to_string()))
    }
   else{
        let user_charms: Vec<Charm> = match query_as!(Charm, "SELECT * FROM charms WHERE user_id = $1", target_user.user_id)
        .fetch_all(pool)
        .await
        {
            Ok(users) => users,
            Err(e) => return Err::<UserProfile, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let charm_count: usize = user_charms.len();
        let following_users: Vec<KleahUserFollows> = match query_as!(KleahUserFollows, "SELECT * FROM user_follows WHERE follower = $1", target_user.user_id)
            .fetch_all(pool)
            .await
        {
            Ok(users) => users,
            Err(e) => return Err::<UserProfile, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let following_users_count: usize = following_users.len();
        let followers_users: Vec<KleahUserFollows> = match query_as!(KleahUserFollows, "SELECT * FROM user_follows WHERE followee = $1", target_user.user_id)
            .fetch_all(pool)
            .await
        {
            Ok(users) => users,
            Err(e) => return Err::<UserProfile, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let followers_users_count: usize = followers_users.len();
        let result: UserProfile = UserProfile{
            user_role: target_user.user_role,
            username: target_user.username,
            display_name: target_user.display_name,
            avatar_url: target_user.avatar_url,
            banner_url: target_user.banner_url,
            user_description: target_user.user_description,
            follower_count: followers_users_count,
            following_count: following_users_count,
            charm_count: charm_count
        };
        Ok(result)
    }
}

pub async fn assemble_files_public(
    payload: &PublicUserViewPayload,
    pool: &Pool<Postgres>
) -> Result<UserFilesTimeline, KleahErr> {
    let target_user: KleahUser = match get_user_by_handle(&payload.username, pool).await {
        Ok(user) => user,
        Err(e) => return Err::<UserFilesTimeline, KleahErr>(KleahErr::new(&e.to_string()))
    };
    if target_user.is_private {
        let e: &str = "You are not allowed to view this content.";
        return Err::<UserFilesTimeline, KleahErr>(KleahErr::new(&e.to_string()))
    }
   else{
        let files: Vec<KleahUserFile> = match query_as!(KleahUserFile, "SELECT * FROM user_files")
            .fetch_all(pool)
            .await
        {
            Ok(files) => files,
            Err(e) => return Err::<UserFilesTimeline, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let mut result: Vec<KleahUserFile> = Vec::new();
        for file in files {
            if file.user_id == target_user.user_id && !(file.is_private) {
                result.push(file);
            }
            else {}
        }
        Ok(UserFilesTimeline{files:result})
    }
}