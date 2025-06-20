/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

use lettre::Message;

/// Importing the error
/// structure to catch
/// and handle errors.
use super::err::KleahErr;

/// Importing the "Tokio1Executor"
/// to execute asynchronous requests.
use lettre::Tokio1Executor;

/// Importing this structure to
/// assist with transporting data.
use lettre::transport::AsyncTransport;

/// Importing this structure to
/// assist with transporting data.
use lettre::transport::smtp::AsyncSmtpTransport;

/// Importing the "Response" structure to
/// handle responses.
use lettre::transport::smtp::response::Response;

/// Importing the "Credentials" to supply credentials.
use lettre::transport::smtp::authentication::Credentials;

/// This function attempts to
/// send an email to the specified
/// sender with the specified parameters.
/// If this operation fails, an error is
/// returned.
pub async fn send_email(
    sender: &String,
    password: &String,
    subject: &String,
    body: &String,
    receiver: &String,
    server: &String
) -> Result<bool, KleahErr> {
    let smtp_credentials = Credentials::new(sender.to_string(), password.to_string());
    let mailer = match AsyncSmtpTransport::<Tokio1Executor>::relay(&server)
    {
        Ok(mailer) => mailer.credentials(smtp_credentials).build(),
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let to = match receiver.parse(){
        Ok(to) => to,
        Err(_e) => return Err::<bool, KleahErr>(KleahErr::new(&"Could not parse receiver.".to_string()))
    };
    let from = match sender.parse(){
        Ok(from) => from,
        Err(_e) => return Err::<bool, KleahErr>(KleahErr::new(&"Could not parse sender.".to_string()))
    };
    let email = match Message::builder()
        .from(from)
        .to(to)
        .subject(subject)
        .body(body.to_string())
    {
        Ok(email) => email,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let send_op: Response = match mailer.send(email).await {
        Ok(send_op) => send_op,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(send_op.is_positive())
}
