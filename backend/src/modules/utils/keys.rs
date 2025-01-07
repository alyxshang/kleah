use pem::encode;
use crate::KleahErr;
use rand::rngs::OsRng;
use rsa::RsaPublicKey;
use rsa::RsaPrivateKey;
use rsa::ToPublicKey;
use rsa::pkcs1::ToRsaPrivateKey;

pub struct KeyPair {
    pub public_key: String,
    pub private_key: String
}

pub fn generate_key_pair() -> Result<KeyPair, KleahErr> {
    let mut rng = OsRng;
    let private_key = match RsaPrivateKey::new(&mut rng, 2048){
        Ok(private_key) => private_key,
        Err(e) => return Err::<KeyPair, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let public_key = RsaPublicKey::from(&private_key);
    let pemmed_priv = match private_key.to_pem_pkcs1(){
        Ok(pemmed_priv) => pemmed_priv,
        Err(e) => return Err::<KeyPair, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let pemmed_pub = match public_key.to_pem(){
        Ok(pemmed_pub) => pemmed_pub,
        Err(e) => return Err::<KeyPair, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let private_key_pem = encode(&pemmed_priv);
    let public_key_pem = encode(&pemmed_pub);
    Ok(
        KeyPair{
            private_key: private_key_pem,
            public_key: public_key_pem
        }
    )
}
