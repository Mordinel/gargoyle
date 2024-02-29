use log::info;
use crate::monitor::{Action, Monitor};

use sysinfo::System;

/// Check the local system for a service by name.
///
/// # Example
///
/// ```
/// # use std::thread::sleep;
/// # use std::time::Duration;
/// use gargoyle::{modules::{monitor, notify}, Schedule};
/// let process_name = "top";
/// let service_monitor = monitor::Service::new(process_name);
/// let stdout_notifier = notify::Stdout;
/// let mut schedule = Schedule::new();
/// schedule.add(
///    &format!("The Gargoyle has detected that {process_name} has gone down"),
///    &format!("The Gargoyle has detected that {process_name} has recovered"),
///    Duration::from_secs(30),
///    &service_monitor,
///    &stdout_notifier,
/// );
///
/// loop {
///    schedule.run();
///    sleep(Duration::from_millis(100));
/// }
/// ```
pub struct Service {
    pub process_name: String,
    system: System,
}

/// Check the local system for a service by exact name.
///
/// # Example
///
/// ```
/// # use std::thread::sleep;
/// # use std::time::Duration;
/// use gargoyle::{modules::{monitor, notify}, Schedule};
/// let process_name = "nginx";
/// let service_monitor = monitor::ExactService::new(process_name);
/// let stdout_notifier = notify::Stdout;
/// let mut schedule = Schedule::new();
/// schedule.add(
///    &format!("The Gargoyle has detected that {process_name} has gone down"),
///    &format!("The Gargoyle has detected that {process_name} has recovered"),
///    Duration::from_secs(60),
///    &service_monitor,
///    &stdout_notifier,
/// );
///
/// loop {
///    schedule.run();
///    sleep(Duration::from_millis(100));
/// }
/// ```
pub struct ExactService {
    pub process_name: String,
    system: System,
}

impl Service {
    pub fn new(process_name: &str) -> Service {
        Service {
            process_name: process_name.to_string(),
            system: System::new(),
        }
    }
}

impl ExactService {
    pub fn new(process_name: &str) -> ExactService {
        ExactService {
            process_name: process_name.to_string(),
            system: System::new(),
        }
    }
}

/// Checks the local system for a service by name.
impl Monitor for Service {
    fn check(&mut self) -> Action {
        self.system.refresh_processes();
        if self.system.processes_by_name(&self.process_name).next().is_none() {
            info!("{} is down", self.process_name);
            Action::Notify(Some(format!("{} is down", self.process_name)))
        } else {
            info!("{} is up", self.process_name);
            Action::Nothing
        }
    }
}

/// Checks the local system for a service by exact name.
impl Monitor for ExactService {
    fn check(&mut self) -> Action {
        self.system.refresh_processes();
        if self.system.processes_by_exact_name(&self.process_name).next().is_none() {
            info!("{} is down", self.process_name);
            Action::Notify(Some(format!("{} is down", self.process_name)))
        } else {
            info!("{} is up", self.process_name);
            Action::Nothing
        }
    }
}

