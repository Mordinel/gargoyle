
/// The `Notify` trait represents the ability to send an alert message.
pub trait Notify: Send + Sync {
    fn send(&self, msg: &str, diagnostic: Option<String>) -> Result<(), String>;
}

