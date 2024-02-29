
/// The `Notify` trait represents the ability to send an alert message.
///
/// # Example
///
/// ```
/// use lettre::{
///     Addres,
///     message::Mailbox,
///     Message, 
///     transport::smtp::authentication::Credentials, 
///     SmtpTransport, 
///     Transport
/// };
/// 
/// use log::info;
/// use gargoyle::Notify;
///
/// pub struct Email {
///     pub from: Mailbox,
///     pub to: Mailbox,
///     pub relay: String,
///     pub smtp_username: String,
///     pub smtp_password: String,
/// }
/// 
/// impl Notify for Email {
///     fn send(&self, msg: &str, diagnostic: Option<String>) -> Result<(), String> {
///         let email = Message::builder()
///             .from(self.from.clone())
///             .to(self.to.clone())
///             .subject(msg)
///             .body(diagnostic.unwrap_or(msg.to_string()))
///             .map_err(|e| format!("Failed to build a message: {e}"))?;
/// 
///         let creds = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());
/// 
///         let mailer = SmtpTransport::relay(&self.relay)
///             .map_err(|e| format!("Failed to create a mailer: {e}"))?
///             .credentials(creds)
///             .build();
/// 
///         info!("Sending email notification from {} to {} via {}.", self.from, self.to, self.relay);
///         match mailer.send(&email) {
///             Ok(_) => Ok(()),
///             Err(e) => Err(format!("Failed to send email: {e}")),
///         }
///     }
/// }
/// ```
pub trait Notify: Send + Sync {
    fn send(&self, msg: &str, diagnostic: Option<String>) -> Result<(), String>;
}

