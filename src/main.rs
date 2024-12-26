/*
Jade by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing this app's
/// tiny CLI function.
use jade::cli;

/// The main point of entry for the
/// Rust compiler.
#[tokio::main]
async fn main() {
    match cli().await {
        Ok(feedback) => println!("{}", feedback),
        Err(e) => println!("{}", &e.to_string())
    }
}