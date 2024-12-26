/*
Jade by Alyx Shang.
Licensed under the FSL v1.
*/

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
/// structure to create directories.
use std::fs::create_dir;

/// Importing this crate's
/// error structure.
use super::err::JadeErr;

/// Importing the "Postgres"
/// structure from the "sqlx"
/// crate.
use sqlx::postgres::Postgres;

/// Saves a file that users upload in the
/// specified directory. If this operation
/// succeeds, the file path is returned.
/// If it fails, an error is returned.
pub async fn save_file(
    bytes: &Vec<u8>, 
    username: &String, 
    folder_name: &String, 
    file_name: &String
) -> Result<String, JadeErr> {
    let mut path_buf: PathBuf = PathBuf::new();
    path_buf.push(folder_name);
    path_buf.push(format!("{}-{}", username, file_name));
    if Path::new(folder_name).exists(){
        let _dir_creation: () = match create_dir(folder_name){
            Ok(_dir_creation) => _dir_creation,
            Err(e) => return Err::<String, JadeErr>(JadeErr::new(&e.to_string()))
        };        
    }
    else {}
    let mut file: File = match File::create(path_buf.display().to_string()){
        Ok(created) => created,
        Err(e) => return Err::<String, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let _write_op: () = match file.write_all(bytes){
        Ok(_write_op) => _write_op,
        Err(e) => return Err::<String, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let file_path: String = path_buf.display().to_string();
    Ok(file_path)
}

/// Attempts to create a connection to a PostgreSQL database given a database
/// URL.
pub async fn create_connection(db_url: &String) -> Result<Pool<Postgres>, JadeErr> {
    let conn = match sqlx::postgres::PgPool::connect(db_url).await{
        Ok(conn) => conn,
        Err(e) => return Err::<Pool<Postgres>, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(conn)
}