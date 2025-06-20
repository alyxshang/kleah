/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Regex"
/// structure to make a new
/// regular expression.
use regex::Regex;

/// Importing this crate's
/// structure for catching
/// and handling errors.
use super::err::KleahErr;

/// Importing the structure
/// to encapsulate info about
/// foreign actors.
use crate::trans::ForeignActor;

/// This function attempts to retrieve the users and their hosts
/// mentioned in a note. If this operation fails, an error is
/// returned.
pub fn get_mentions(note: &str) -> Result<Vec<ForeignActor>, KleahErr>{
    let mut result: Vec<ForeignActor> = Vec::new();
    let regex: Regex = match Regex::new(r"(@[a-zA-Z0-9]+@[a-zA-Z0-9]+\.[a-zA-Z0-9]+)"){
        Ok(regex) => regex,
        Err(e) => return Err::<Vec<ForeignActor>,KleahErr>(KleahErr::new(&e.to_string()))
    };
    for captured in regex.captures(note).iter(){
        let cap = match captured.get(1){
            Some(cap) => cap,
           None => return Err::<Vec<ForeignActor>,KleahErr>(KleahErr::new(&"No capture found.".to_string()))
        };
        let mut group = cap.as_str().to_string();
        group.remove(0);
        let split_group: Vec<&str> = group.split("@").collect();
        result.push(
            ForeignActor{
                host: split_group[1].to_string(),
                user: split_group[0].to_string()
            }
        );
    }
    Ok(result)
}