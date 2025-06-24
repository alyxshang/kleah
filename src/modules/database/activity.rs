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

/// Importing the "query_as"
/// macro to execute queries
/// that return something.
use sqlx::query_as;

/// Importing the "Postgres"
/// structure for explicit 
/// typing.
use sqlx::postgres::Postgres;

use crate::err::KleahErr;
/// Importing the "TimeNow" structure to get
/// the current time.
use crate::modules::units::trans::TimeNow;

/// Importing the function
/// to generate a hash from
/// a string.
use crate::modules::utils::hash_string_sha;

use crate::modules::units::models:Activity;

/// Importing the structure
/// from the database for
/// casting the type of a
/// private actor explicitly.
use crate::modules::units::models::ActorFile;

use crate::modules::units::models::StreamedActivity;

use crate::modules::database::token::get_actor_by_token;

pub async fn create_streamed_activity(
    api_token: &str,

    activity_type: String,
) -> Result<StreamedActivity, KleahErr>{
    let user: Actor = get_actor_by_token(api_token, pool){
        Ok(user) => user,
        Err(e) => return Err::<StreamedActivity, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let id: String = format!("{}{}{}", 
    let streamed: StreamedActivity = StreamedActivity{
        activity_id: 
        activity_type: String,
        actor_id: 
        published: TimeNow::new().to_string()

    };
}

pub async fn get_streamed_activity_by_id(
) -> Result<StreamedActivity, KleahErr>{
}

pub async fn delete_streamed_activity(
) -> Result<StreamedActivity, KleahErr>{
}

pub async fn create_activity(
) -> Result<StreamedActivity, KleahErr>{
}

pub async fn get_activity_by_id(
) -> Result<StreamedActivity, KleahErr>{
}

pub async fn delete_activity(
) -> Result<StreamedActivity, KleahErr>{
}
