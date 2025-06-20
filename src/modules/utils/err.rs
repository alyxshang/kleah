/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the standard
/// "Result" enum.
use std::fmt::Result;

/// Importing the
/// macro for 
/// serializing Rust
/// data structures
/// as JSON.
use serde::Serialize;

/// Importing the standard
/// "Display" trait.
use std::fmt::Display;

/// Importing the standard
/// "Error" trait.
use std::error::Error;

/// Importing the standard
/// "Formatter" trait.
use std::fmt::Formatter;

/// Importing the "HttpResponse"
/// structure to return a HTTP
/// response.
use actix_web::HttpResponse;

/// Importing the "StatusCode"
/// enum to return a status
/// code for a failed request.
use actix_web::http::StatusCode;

/// Importing the "ResponseErr"
/// trait to implement it for the
/// "KleahErr" structure.
use actix_web::error::ResponseError;

/// A data structure
/// to create JSON
/// responses for failed
/// requests.
#[derive(Serialize)]
pub struct ErrDetails{
    pub error: String
}

/// A data structure for
/// storing and handling errors.
#[derive(Clone,Eq,PartialEq, Debug)]
pub struct KleahErr {
    pub details: String
}

/// Implements generic methods.
impl KleahErr {

    /// Implements a generic method to create
    /// a new instance of this data structure.
    pub fn new(details: &str) -> KleahErr {
        KleahErr {
            details: details.to_owned()
        }
    }

    /// Implements a generic method to return
    /// a string representation of this 
    /// data structure.
    pub fn to_string(self) -> String {
        return self.details.to_string();
    }
}

/// Implements the error trait.
impl Error for KleahErr {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Implements the Display trait.
impl Display for KleahErr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(f,"{}",self.details);
    }
}

/// Implementing the "ResponseError"
/// trait for the "KleahErr" structure.
impl ResponseError for KleahErr {

    /// Returns error details as a JSON
    /// response.
    fn error_response(&self) -> HttpResponse {
        let e: ErrDetails = ErrDetails { error: (&self.details).clone() };
        HttpResponse::Ok().json(e)
    }

    /// Returns a status code for a failed
    /// HTTP request.
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

}
