use log::{info, error};
use reqwest::blocking::Client;
use crate::monitor::{Action, Monitor};


/// The `WebAvailability` struct represents a monitor that checks the availability of a 
/// web service.
pub struct WebAvailability {
    pub url: String,
    pub web_client: Client,
}

impl WebAvailability {
    pub fn new(url: &str) -> Self {
        let web_client = Client::new();
        Self {
            url: url.to_string(),
            web_client,
        }
    }
}

/// Implement the `Monitor` trait for `WebAvailability`.
impl Monitor for WebAvailability {
    /// Check the availability of the web service.
    fn check(&self) -> Action {
        match self.web_client.get(&self.url).send() {
            Ok(response) => {
                if response.status().is_success() {
                    info!("{} is up", self.url);
                    Action::Nothing
                } else {
                    info!("{} is down", self.url);
                    error!("Failed to get {} - {}", self.url, response.status());
                    Action::Notify(Some(format!("Failed to get {} - {}", self.url, response.status())))
                }
            }
            Err(_) => {
                info!("{} is down", self.url);
                error!("Failed to connect to {}", self.url);
                Action::Notify(Some(format!("Failed to connect to {}", self.url)))
            }
        }
    }
}

