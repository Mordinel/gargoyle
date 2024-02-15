pub mod local;
pub mod web;

/// The `Action` enum represents the possible actions that can occur 
/// as a result of a monitor check.
pub enum Action {
    Alert(Option<String>),
    Nothing,
}

/// The `Monitor` trait represents the ability to monitor a system for a specific condition.
pub trait Monitor: Send + Sync {
    fn check(&self) -> Action;
}

