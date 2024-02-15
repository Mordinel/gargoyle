mod email;
mod stdout;

pub use email::Email;
pub use stdout::Stdout;
pub use lettre::{
    message::Mailbox,
    Address,
};

/// The `Alert` trait represents the ability to send an alert message.
pub trait Alert: Send + Sync {
    fn send(&self, msg: &str, diagnostic: Option<String>) -> Result<(), String>;
}

