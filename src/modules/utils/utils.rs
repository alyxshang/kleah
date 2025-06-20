/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// "Pem" structure
/// to generate keys
/// in the PEM format.
use pem::Pem;

/// Importing the
/// "encode" function
/// to encode a byte
/// array in PEM format.
use pem::encode;

/// Importing the "Digest"
/// trait from the "sha2"
/// crate.
use sha2::Digest;

/// Importing the "Sha256"
/// structure from the "sha2"
/// crate to process strings.
use sha2::Sha256;

/// Importing the
/// "hash" function
/// to hash strings.
use bcrypt::hash;

/// Importing the
/// "verify" function
/// to verify hashed
/// strings.
use bcrypt::verify;

/// Importing the
/// "Rsa" structure
/// to generate keys.
use openssl::rsa::Rsa;

/// Importing this crate's
/// structure for catching
/// and handling errors.
use super::err::KleahErr;

/// Importing the
/// "DEFAULT_COST" enum
/// to specify how to hash
/// a string.
use bcrypt::DEFAULT_COST;

/// Importing the structure
/// to save info about an
/// actor's keys.
use crate::trans::ActorKeys;

/// Creates and returns the SHA-256 sum
/// of the supplied string.
pub fn hash_string_sha(subject: &str) -> String {
    let mut hasher: Sha256 = Sha256::new();
    hasher.update(subject.to_string());
    format!("{:X}", hasher.finalize())
}

/// This function attempts to generate a pair of keys for
/// an actor and return this info in the "ActorKeys" structure
/// for encapsulation. If this fails, an error is returned.
pub fn generate_keys() -> Result<ActorKeys, KleahErr>{
    let key_pair = match Rsa::generate(4096){
        Ok(key_pair) => key_pair,
        Err(e) => return Err::<ActorKeys, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let priv_key: String = match key_pair.private_key_to_der(){
        Ok(priv_key) => encode(&Pem::new(String::from("PRIVATE KEY"), priv_key)),
        Err(e) => return Err::<ActorKeys, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let public_key: String = match key_pair.public_key_to_der(){
        Ok(public_key) => encode(&Pem::new(String::from("PUBLIC KEY"), public_key)),
        Err(e) => return Err::<ActorKeys, KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(ActorKeys{ private: priv_key, public: public_key })
}

/// This function attempts
/// to hash a string and 
/// return the hash. If this
/// operation fails, an error
/// is returned.
pub fn hash_string(
    subject: &str
) -> Result<String, KleahErr>{
    let hashed = match hash(subject, DEFAULT_COST){
        Ok(hashed) => hashed,
        Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(hashed)
}

/// This function attempts
/// to verify a hashed string and 
/// return the result. If this
/// operation fails, an error
/// is returned.
pub fn verify_string(
    subject: &String,
    against: &String
) -> Result<bool, KleahErr>{
    let valid = match verify(against, subject){
        Ok(valid) => valid,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(valid)
}
