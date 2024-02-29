use log::info;
use crate::monitor::{Action, Monitor};

use sysinfo::System;

/// Check the local system for a service by name.
pub struct Service {
    pub process_name: String,
}

/// Check the local system for a service by exact name.
pub struct ExactService {
    pub process_name: String,
}

impl Service {
    /// Create a new `Service` struct.
    pub fn new(process_name: &str) -> Service {
        Service {
            process_name: process_name.to_string(),
        }
    }
}

impl ExactService {
    /// Create a new `ExactService` struct.
    pub fn new(process_name: &str) -> ExactService {
        ExactService {
            process_name: process_name.to_string(),
        }
    }
}

/// Implement the `Monitor` trait for the `Service` struct.
impl Monitor for Service {
    /// Check the local system for a service by name.
    fn check(&self) -> Action {
        let mut system = System::new_all();
        system.refresh_processes();
        if system.processes_by_name(&self.process_name).next().is_none() {
            info!("{} is down", self.process_name);
            Action::Notify(Some(format!("{} is down", self.process_name)))
        } else {
            info!("{} is up", self.process_name);
            Action::Nothing
        }
    }
}

/// Implement the `Monitor` trait for the `ExactService` struct.
impl Monitor for ExactService {
    /// Check the local system for a service by exact name.
    fn check(&self) -> Action {
        let mut system = System::new_all();
        system.refresh_processes();
        if system.processes_by_exact_name(&self.process_name).next().is_none() {
            info!("{} is down", self.process_name);
            Action::Notify(Some(format!("{} is down", self.process_name)))
        } else {
            info!("{} is up", self.process_name);
            Action::Nothing
        }
    }
}

