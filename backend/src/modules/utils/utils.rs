/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the
/// range API
/// from the "rand"
/// crate.
use rand::Rng;

use reqwest::Client;

use serde_json::from_str;

use reqwest::header::ACCEPT;

use reqwest::header::CONTENT_TYPE;

/// Importing the
/// "Pool" structure
/// from the "sqlx" crate
/// to make a pool for
/// database connections.
use sqlx::Pool;

/// Importing the "File" structure
/// to use it.
use std::fs::File;

/// Importing the "Write"
/// trait to use it for
/// writing to a file.
use std::io::Write;

use crate::responses::WebFingerInfo;
/// Importing this crate's
/// error structure.
use crate::KleahErr;

/// Importing the "Path"
/// structure to check
/// whether the target
/// directory exists or not.
use std::path::Path;

/// Importing the "PathBuf"
/// structure to check
/// paths and create them.
use std::path::PathBuf;

/// Importing the "create_dir"
/// function to create directories
/// to store user-uploaded files
/// in.
use std::fs::create_dir;

/// Importing the "remove_file"
/// function to delete 
/// user-uploaded files.
use std::fs::remove_file;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// Attempts to delete a file on
/// disk. Retruns a boolean "true"
/// if successful. If the operation
/// fails, an error is returned.
pub async fn delete_file(
    file_path: &String,
) -> Result<bool, KleahErr> {
    let result: bool;
    let _del_op: () = match remove_file(Path::new(file_path)){
        Ok(_del_op) => result = true,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(result)
}

/// Saves a file that users upload in the
/// specified directory. If this operation
/// succeeds, the file path is returned.
/// If it fails, an error is returned.
pub async fn save_file(
    bytes: &Vec<u8>, 
    username: &String, 
    folder_name: &String, 
    file_name: &String
) -> Result<String, KleahErr> {
    let mut path_buf: PathBuf = PathBuf::new();
    path_buf.push(folder_name);
    path_buf.push(format!("{}-{}", username, file_name));
    if Path::new(folder_name).exists(){
        let _dir_creation: () = match create_dir(folder_name){
            Ok(_dir_creation) => _dir_creation,
            Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
        };        
    }
    else {}
    let mut file: File = match File::create(path_buf.display().to_string()){
        Ok(created) => created,
        Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let _write_op: () = match file.write_all(bytes){
        Ok(_write_op) => _write_op,
        Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let file_path: String = path_buf.display().to_string();
    Ok(file_path)
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

/// Generates a random character sequence of a 
/// specified length and returns the resulting
/// string.
pub fn generate_chars(length: usize) -> String {
    let range_end: usize;
    if length == 0 {
        range_end = 8;
    }
    else {
        range_end = length + 1;
    }
    let mut result_chars: Vec<char> = Vec::new();
    let alphabet: Vec<char> = "1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    for _i in 1..range_end {
        let mut range = rand::thread_rng();
        let rand_char: char = alphabet[range.gen_range(0..alphabet.len())].clone();
        result_chars.push(rand_char);
    }
    let result: String = result_chars.into_iter().collect();
    result
}

pub async fn get_webfinger_info_from_other_instance(
    host: &String, 
    username: &String
) -> Result<WebFingerInfo, KleahErr> {
    let client: Client = Client::new();
    let url: String = format!("https://{}/.well-known/webfinger?resource=acct:{}@{}", host, username, host);
    let response = match client.get(url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/activity+json")
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => return Err::<WebFingerInfo, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let resp_text: String = match response.text().await {
        Ok(resp_text) => resp_text,
        Err(e) => return Err::<WebFingerInfo, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let result: WebFingerInfo = match from_str(&resp_text){
        Ok(result) => result,
        Err(e) => return Err::<WebFingerInfo, KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(result)
}