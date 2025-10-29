/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the namespace
/// for deriving some of
/// Actix Web's native error-
/// handling entities.
use actix_web::error;

/// Importing the macro
/// to serialize Rust 
/// data structures into
/// JSON.
use serde::Serialize;

/// Importing the "Result"
/// entity from the standard
/// library.
use std::fmt::Result;

/// Importing the "Display"
/// trait from the standard
/// library to implement it.
use std::fmt::Display;

/// Importing the "Error"
/// trait from the standard
/// library to implement it.
use std::error::Error;

/// Importing the "Error"
/// trait from the standard
/// library to implement it.
use std::fmt::Formatter;

/// Importing the "Httpresponse"
/// structure to return errors
/// as HTTP responses.
use actix_web::HttpResponse;

/// A structure to encapsulate
/// error information and serialize
/// this information into a JSON
/// string.
#[derive(Serialize)]
pub struct ErrDetails {
    pub details: String
}

/// A structure to capture and
/// handle errors.
#[derive(Clone,Eq,PartialEq, Debug)]
pub struct KleahErr{
    pub details: String
}

/// Defining some useful functions
/// for the `KleahErr` structure.
impl KleahErr{

    /// Implementing a function to create
    /// a new instance of the `KleahErr`
    /// structure and return it.
    pub fn new(details: &str) -> KleahErr {
        KleahErr{
            details: details.to_owned()
        }
    }

    /// Implementing a function to create
    /// a string representation of the 
    /// current `KleahErr` instance
    /// and return it.
    pub fn to_string(self) -> String {
        self.details.to_string()
    }
}

/// Implementing the standard
/// `Error` trait for the `KleahErr`
/// structure.
impl Error for KleahErr {

    /// The function that implements
    /// this `Error` trait.
    fn description(
        &self
    ) -> &str {
        &self.details
    }

}

/// Implementing the standard
/// `Display` trait for the `KleahErr`
/// structure.
impl Display for KleahErr{

    /// The function that implements
    /// this `Display` trait.
    fn fmt(
        &self, 
        f: &mut Formatter
    ) -> Result {
        write!(f,"{}",self.details)
    }
}

/// Implementing the `ResponseError`
/// trait for the `KleahErr`
/// structure.
impl error::ResponseError for KleahErr {

    /// The function that implements
    /// this `ResponseError` trait.
    fn error_response(&self) -> HttpResponse {
        let resp: ErrDetails = ErrDetails{ 
            details: (*((&self.details).clone())).to_string()
        };
        HttpResponse::Ok().json(resp)
    }
}
