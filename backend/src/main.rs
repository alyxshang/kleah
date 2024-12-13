/*
Kleah Backend by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the backend's
/// CLI function.
use kleah_backend::cli;

/// The main point
/// of entry for
/// the Rust compiler
/// and Actix Web.
#[actix_web::main]
async fn main(){
    match cli().await{
        Ok(feedback) => println!("{}", feedback),
        Err(e) => eprintln!("{}", e.to_string())
    };
}