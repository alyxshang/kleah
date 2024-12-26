/*
Jade by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Message"
/// structure to send a message.
use lettre::Message; 

/// Importing the "Transport"
/// entity to specify the 
/// transport protocol.
use lettre::Transport;

/// Importing this crate's
/// error structure.
use super::err::JadeErr;

/// Importing the "SmtpTransport"
/// entitiy to specify the 
/// transport protocol.
use lettre::SmtpTransport;

/// Attempts to send an email. If the
/// operation succeeds, a boolean "true"
/// is returned. If the operation fails,
/// an error is returned.
pub async fn send_email(
    from: &String,
    to: &String,
    subject: &String,
    msg: &String,
    server: &String
) -> Result<bool, JadeErr>{
    let mut res: bool = false;
    let parsed_from = match from.parse(){
        Ok(parsed_from) => parsed_from,
        Err(_e) => {
            let e: String = format!("Could not parse sender: \"{}\"", from);
            return Err::<bool, JadeErr>(JadeErr::new(&e.to_string()))
        }
    };
    let parsed_to = match to.parse(){
        Ok(parsed_to) => parsed_to,
        Err(_e) => {
            let e: String = format!("Could not parse sender: \"{}\"", to);
            return Err::<bool, JadeErr>(JadeErr::new(&e.to_string()))
        }
    };
    let email = match Message::builder()
        .from(parsed_from)
        .to(parsed_to)
        .subject(subject)
        .body(msg.to_string())
    {
        Ok(email) => email,
        Err(e) => return Err::<bool, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let sender = match SmtpTransport::relay(server){
        Ok(sender) => sender,
        Err(e) => return Err::<bool, JadeErr>(JadeErr::new(&e.to_string()))
    };
    let _result = match sender.build().send(&email){
        Ok(_sender) => res = true,
        Err(e) => return Err::<bool, JadeErr>(JadeErr::new(&e.to_string()))
    };
    Ok(res)
}
