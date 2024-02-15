use log::{info, error};
use reqwest::blocking::Client;
use super::{Action, Monitor};


/// The `Availability` struct represents a monitor that checks the availability of a web service.
pub struct Availability {
    pub url: String,
    pub web_client: Client,
}

impl Availability {
    pub fn new(url: &str) -> Self {
        let web_client = Client::new();
        Availability {
            url: url.to_string(),
            web_client,
        }
    }
}

/// Implement the `Monitor` trait for `Availability`.
impl Monitor for Availability {
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
                    Action::Alert(Some(format!("Failed to get {} - {}", self.url, response.status())))
                }
            }
            Err(_) => {
                info!("{} is down", self.url);
                error!("Failed to connect to {}", self.url);
                Action::Alert(Some(format!("Failed to connect to {}", self.url)))
            }
        }
    }
}

