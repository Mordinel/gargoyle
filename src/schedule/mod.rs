use log::error;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use super::{alert::Alert, monitor::{Monitor, Action}};

/// The `Entry` struct represents a scheduled check of a monitor.
struct Entry<'a> {
    /// The message to send if the monitor fires.
    fire_message: String,
    /// The message to send if the monitor recovers.
    recover_message: String,
    /// The last time the monitor was checked.
    last_checked: Option<Instant>,
    /// The amount of time to wait between checks.
    wait_time: Duration,
    /// The monitor to check.
    monitor: &'a dyn Monitor,
    /// The alert to send if the monitor fires.
    alert: &'a dyn Alert,
    /// Whether the monitor has fired.
    has_fired: bool,
}

/// The `Schedule` struct represents a collection of scheduled checks.
#[derive(Default)]
pub struct Schedule<'a> {
    entries: Vec<Arc<Mutex<Entry<'a>>>>,
}

/// Implement the `Schedule` struct.
impl<'a> Schedule<'a> {
    /// Add a new entry to the `Schedule` instance.
    pub fn add<M: Monitor, A: Alert>(
        mut self, 
        fire_message: &str,
        recover_message: &str,
        wait_time: Duration, 
        monitor: &'a M, 
        alert: &'a A
    ) -> Schedule<'a> {
        self.entries.push(Arc::new(Mutex::new( Entry {
            fire_message: fire_message.to_string(),
            recover_message: recover_message.to_string(),
            last_checked: None,
            wait_time,
            monitor,
            alert,
            has_fired: false,
        })));
        self
    }

    /// Run the `Schedule` instance and check all the monitors for alerts or recoveries.
    pub fn run(&mut self) {
        for entry in &mut self.entries {
            rayon::scope(|s| {
                s.spawn(|_| {
                    // If the `Monitor` has not been checked,
                    // or if the `Monitor` has not been checked in the specified amount of time,
                    // check the `Monitor`.
                    let mut entry = entry.lock().unwrap();
                    if entry.last_checked.is_none() || entry.last_checked.unwrap().elapsed() >= entry.wait_time {
                        entry.last_checked = Some(Instant::now());
                        match entry.monitor.check() {
                            Action::Alert(diagnostic) => {
                                if !entry.has_fired {
                                    // If the `Alert` fails to send the message, log the error
                                    // and don't update the `has_fired` flag, this will cause the 
                                    // `Alert` to be sent again on the next iteration.
                                    //
                                    // TODO: Add a retry mechanism for alert sending.
                                    if let Err(e) = entry.alert.send(&entry.fire_message, diagnostic) {
                                        error!("{e}");
                                    } else {
                                        entry.has_fired = true;
                                    }
                                }
                            }
                            Action::Nothing => {
                                if entry.has_fired {
                                    if let Err(e) = entry.alert.send(&entry.recover_message, None) {
                                        error!("{e}");
                                    } else {
                                        entry.has_fired = false;
                                    }
                                }
                            }
                        }
                    }
                });
            });
        }
    }
}

