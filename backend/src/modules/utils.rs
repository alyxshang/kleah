/*
Kleah Backend by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// "Pool" structure
/// from the "sqlx" crate
/// to make a pool for
/// database connections.
use sqlx::Pool;

/// Importing the "Digest"
/// trait from the "sha2"
/// crate.
use sha2::Digest;

/// Importing the "Sha256"
/// structure from the "sha2"
/// crate to process strings.
use sha2::Sha256;

/// Importing this crate's
/// error structure.
use super::err::KleahErr;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// Creates and returns the SHA-256 sum
/// of the supplied string.
pub fn hash_string(link: &String) -> String {
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(link);
    format!("{:X}", hasher.finalize())
}

/// Generates the ID of every link submitted
/// by taking the last four characters of a 
/// SHA-256 sum.
pub fn generate_id(shasum: &String) -> String {
    let shasum_chars: Vec<char> = shasum.chars().collect();
    let start: usize = shasum_chars.len() - 4;
    let end: usize = shasum_chars.len();
    let mut result_vec: Vec<String> = Vec::new();
    for i in start..end {
        result_vec.push(shasum_chars[i].to_string());
    }
    let result: String = result_vec.join("");
    result
}

/// Attempts to create a connection to a PostgreSQL database given a database
/// URL.
pub async fn create_connection(db_url: &String) -> Result<Pool<Postgres>, KleahErr> {
    let conn = match sqlx::postgres::PgPool::connect(db_url).await{
        Ok(conn) => conn,
        Err(e) => return Err::<Pool<Postgres>, KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(conn)
}