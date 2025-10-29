/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the structure
/// representing a pool of
/// database connections.
use sqlx::Pool;

/// Importing this
/// namespace to use
/// a type alias.
use sqlx::postgres;

/// Importing the structure
/// for generating an RSA
/// keypair.
use openssl::rsa::Rsa;

/// Importing the structure
/// for catching and handling
/// errors.
use super::err::KleahErr;

/// Importing the data
/// structure holding
/// information about a pair
/// of public and private keys, 
/// respectively.
use super::units::KeyPair;

/// Importing the entity that
/// represents a private RSA key.
use openssl::pkey::Private;

/// Importing the structure that
/// represents a connection to a 
/// PostgreSQL database.
use sqlx::postgres::Postgres;


/// A function to check whether
/// the supplied string is a valid
/// username. A boolean relfecting
/// this is returned.
pub fn check_username(
    sub: &str
) -> bool {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxxyz1234567890_"
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    let mut result: bool = true;
    let sub_chars: Vec<char> = sub
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    if sub_chars.len() <= 4{
        result = false;
    }
    else {
        for sub_char in sub_chars{
            if !alphabet.contains(&sub_char){
                result = false;
            }
        }
    }
    result
}

/// A function to check whether
/// the supplied string is a valid
/// password. A boolean relfecting
/// this is returned.
pub fn check_password(
    sub: &str
) -> bool {
    let sub_chars: Vec<char> = sub
        .to_string()
        .chars()
        .collect::<Vec<char>>();
    sub_chars.len() > 8 && sub_chars.len() <= 16
}

/// A function to check whether
/// the supplied string is a valid
/// email address. A boolean relfecting
/// this is returned.
pub fn check_email(
    sub: &str
) -> bool {
    let split_at: Vec<&str> = sub.split("@")
        .into_iter()
        .collect::<Vec<&str>>();
    split_at.len() == 2
}

/// A function that attempts
/// to generate a public and 
/// private pair of keys for 
/// a user. If the operation
/// is successful, an instance
/// of the `KeyPair` structure
/// is returned. If the operation
/// fails, an error is returned.
pub fn generate_keypair(
) -> Result<KeyPair, KleahErr>{
    let keys: Rsa<Private> = match Rsa::generate(2048){
        Ok(keys) => keys,
        Err(e) => return Err::<KeyPair, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let public_pem: Vec<u8> = match keys.public_key_to_pem_pkcs1(){
        Ok(public_pem) => public_pem,
        Err(e) => return Err::<KeyPair, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let private_pem: Vec<u8> = match keys.private_key_to_pem(){
        Ok(private_pem) => private_pem,
        Err(e) => return Err::<KeyPair, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let public: String = match String::from_utf8(public_pem){
        Ok(public) => public,
        Err(e) => return Err::<KeyPair, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let private: String = match String::from_utf8(private_pem){
        Ok(public) => public,
        Err(e) => return Err::<KeyPair, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let pair: KeyPair = KeyPair{ 
        public_key: public, 
        private_key: private
    };
    Ok(pair)
}

/// A function that attempts to create
/// a connection with a database given the
/// database's URL. If the operation is successful,
/// an instance of the `Pool<Postgres>` structure is
/// returned. If the operation fails, an error is returned.
pub async fn create_connection(
    db_url: &String
) -> Result<Pool<Postgres>, KleahErr> {
    let conn = match postgres::PgPool::connect(
        db_url
    ).await{
        Ok(conn) => conn,
        Err(e) => return Err::<Pool<Postgres>, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(conn)
}
