/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Url"
/// structure to parse
/// URLs.
use url::Url;

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

/// Importing the
/// structure
/// to get the current
/// time.
use chrono::Local;

/// Get the current
/// UTC offset.
use chrono::Offset;

/// Importing the prelude
/// of the "rand" crate
/// to use RSA.
use rand::prelude::*;

/// Importing the structure
/// to generate a public key.
use rsa::RsaPublicKey;

/// Importing the structure
/// to generate a private key.
use rsa::RsaPrivateKey;

/// Importing the data structure
/// to catch and handle errors
/// as values.
use super::err::KleahErr;

/// Importing the data structure
/// to save a keypair.
use super::units::KeyPair;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// Importing the trait to encode
/// into PEM format.
use pkcs1::EncodeRsaPrivateKey;

/// Importing the trait to encode
/// into PEM format.
use pkcs1::EncodeRsaPublicKey;

/// Importing the structure
/// to create a map.
use std::collections::HashMap;


pub fn parse_query(
    url_str: &String
) -> Result<HashMap<String, String>, KleahErr>{
    let mut res: HashMap<String, String> = HashMap::new();
    let parsed = match Url::parse(url_str){
        Ok(parsed) => parsed,
        Err(e) => return Err::<HashMap<String,String>, KleahErr>(
            KleahErr::new(&e.to_string())
        ) 
    };
    let mut counter: usize = 0;
    while let Some(item) = parsed.query(){
        res.insert(format!("{:?}", counter), item.to_string());
        counter += 1;
    }
    Ok(res)
}

/// Creates and returns the SHA-256 sum
/// of the supplied string.
pub fn hash_string(subject: &String) -> String {
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(subject);
    format!("{:X}", hasher.finalize())
}

/// Attempts to create a connection to a PostgreSQL database given a database
/// URL. If this operation fails, an error is returned.
pub async fn create_connection(
    db_url: &String
) -> Result<Pool<Postgres>, KleahErr> {
    let conn = match sqlx::postgres::PgPool::connect(db_url).await{
        Ok(conn) => conn,
        Err(e) => return Err::<Pool<Postgres>, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    Ok(conn)
}

/// A generic structure to
/// hold information on the current
/// local time.
pub struct TimeNow{
    pub year: String,
    pub month: String,
    pub day: String,
    pub hours: String,
    pub minutes: String,
    pub seconds: String,
    pub offset: String
}

/// Implementing generic
/// methods for the "TimeNow"
/// structure.
impl TimeNow{

    /// Implementing a "new"
    /// method for the "TimeNow"
    /// structure.
    pub fn new() -> TimeNow {
        let time = Local::now();
        let date = time.date_naive();
        let curr_time = time.time();
        let year: String = format!("{}",date.format("%Y"));
        let month: String = format!("{}",date.format("%m"));
        let day: String = format!("{}",date.format("%d"));
        let hours: String = format!("{}",curr_time.format("%H"));
        let minutes: String = format!("{}",curr_time.format("%M"));
        let seconds: String = format!("{}",curr_time.format("%S"));
        let offset: String = time.offset().fix().to_string();
        TimeNow {
            year,
            month,
            day,
            hours,
            minutes,
            seconds,
            offset
        }
    }
    
    /// Implementing a generic function
    /// to return a string representation
    /// of this structure.
    pub fn to_string(&self) -> String {
        format!(
            "{}-{}-{}T{}{}{}{}",
            &self.year, 
            &self.month, 
            &self.day, 
            &self.hours, 
            &self.minutes, 
            &self.seconds,
            &self.offset
        )
    }
}

/// A function to generate a private and public
/// keypair for a user. If the operation is successful,
/// an instance of the `KeyPair` structure. If this operation
/// fails, an error is returned.
pub fn generate_keypair() -> Result<KeyPair, KleahErr>{
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let private_key: RsaPrivateKey = match RsaPrivateKey::new(&mut rng, bits){
        Ok(private_key) => private_key,
        Err(e) => return Err::<KeyPair, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let public_key: RsaPublicKey = RsaPublicKey::from(&private_key);
    let public_pem: String = match public_key.to_pkcs1_pem(
        rsa::pkcs1::LineEnding::LF
    ){
        Ok(public_pem) => format!("{:?}",public_pem),
        Err(e) => return Err::<KeyPair, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let private_pem: String = match private_key.to_pkcs1_pem(
        rsa::pkcs1::LineEnding::LF
    ){
        Ok(private_pem) => format!("{:?}",private_pem),
        Err(e) => return Err::<KeyPair, KleahErr>(
            KleahErr::new(&e.to_string())
        )
    };
    let result: KeyPair = KeyPair{
        public: public_pem,
        private: private_pem
    };
    Ok(result)
}

/// A function to
/// remove http-attributes
/// from a host string and
/// return the stripped string.
pub fn parse_host(
    subject: &String
) -> Result<String, KleahErr>{
    if subject.starts_with("http://"){
        Ok(
            subject
                .split("http://")
                .collect::<Vec<&str>>()[1]
                .to_string()
        )
    }
    else if subject.starts_with("https://"){
        Ok(
            subject
                .split("https://")
                .collect::<Vec<&str>>()[1]
                .to_string()
        )
    }
    else {
        Err::<String, KleahErr>(
            KleahErr::new("Unable to parse string.")
        )
    }
}
