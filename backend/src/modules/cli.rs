/*
Kleah Backend by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "App"
/// structure to create
/// a new CLI app.
use cliply::App;

/// Importing the "var"
/// function to retrieve
/// environment variables.
use std::env::var;

/// Importing this crate's
/// error structure.
use super::err::KleahErr;

/// Importing the "run_app"
/// function to run the backend
/// services.
use super::runner::run_app;

/// Importing the "ConfigData"
/// structure for explicit typing.
use super::units::ConfigData;

/// The function containing this app's
/// tiny CLI.
pub async fn cli() -> Result<String, KleahErr>{
    let result: String;
    let mut kleah: App = App::new(
        "Kleah",
        "0.1.0",
        "Alyx Shang"
    );
    kleah.add_arg("runa", "run the application", &false);
    if kleah.version_is(){
        result = kleah.version_info();
    }
    else if kleah.help_is(){
        result = kleah.help_info();
    }
    else if kleah.arg_was_used("runa"){
        let db_url: String = match var("DATABASE_URL"){
            Ok(db_url) => db_url,
            Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let host: String = match var("ACTIX_HOST"){
            Ok(host) => host,
            Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let port: String = match var("ACTIX_PORT"){
            Ok(port) => port,
            Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
        };
        let config: ConfigData = ConfigData::new(
            &db_url,
            &host,
            &port
        );
        let _runner: () = match run_app(&config).await{
            Ok(_runner) => _runner,
            Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
        };
        result = format!("App running on \"{}:{}\".", &host, &port);
    }
    else {
        result = kleah.help_info();
    }
    Ok(result)
}