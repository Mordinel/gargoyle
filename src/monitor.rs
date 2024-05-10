
/// The `Action` enum represents the possible actions that can occur 
/// as a result of a monitor check.
pub enum Action {
    /// `Update` Fires a message every time it is sent.
    Update { message: Option<String> },
    /// `Notify` Fires when a monitor changes from sending `Nothing` to `Notify` and vice versa.
    Notify { diagnostic: Option<String> },
    /// `Nothing` only fires a message if a `Notify` has already fired.
    Nothing,
}

/// The `Monitor` trait represents the ability to monitor a system for a specific condition.
///
/// # Example
///
/// ```
/// use log::{info, error};
/// pub use reqwest::blocking::Client;
/// use gargoyle::{Action, Monitor};
/// 
/// pub struct WebAvailability {
///     pub url: String,
///     web_client: Client,
/// }
/// 
/// impl WebAvailability {
///     pub fn new(url: &str) -> Self {
///         let web_client = Client::builder()
///             .user_agent("Gargoyle/0.1")
///             .build()
///             .unwrap();
///         Self {
///             url: url.to_string(),
///             web_client,
///         }
///     }
/// }
/// 
/// impl Monitor for WebAvailability {
///     fn check(&mut self) -> Action {
///         match self.web_client.get(&self.url).send() {
///             Ok(response) => {
///                 if response.status().is_success() {
///                     info!("{} is up", self.url);
///                     Action::Nothing
///                 } else {
///                     info!("{} is down", self.url);
///                     error!("Failed to get {} - {}", self.url, response.status());
///                     Action::Notify(Some(format!("Failed to get {} - {}", self.url, response.status())))
///                 }
///             }
///             Err(_) => {
///                 info!("{} is down", self.url);
///                 error!("Failed to connect to {}", self.url);
///                 Action::Notify(Some(format!("Failed to connect to {}", self.url)))
///             }
///         }
///     }
/// }
/// ```
pub trait Monitor: Send + Sync {
    fn check(&mut self) -> Action;
}

