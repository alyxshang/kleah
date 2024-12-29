/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the entities
/// for handling errors for
/// Actix Web.
use actix_web::error;

/// Importing the standard
/// "Result" enum.
use std::fmt::Result;

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
/// structure to send an error
/// as a HTTP Response.
use actix_web::HttpResponse;

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

/// Implements the "ResponseError" trait.
impl error::ResponseError for KleahErr {

    /// Implementing the function that is supposed
    /// to handle errors coming from Actix Web.
    fn error_response(&self) -> HttpResponse {
        let json_err: String = format!("{{\"error\":{}}}", &self.details);
        HttpResponse::Ok()
        .content_type("application/json")
        .body(json_err)
    }
    
}