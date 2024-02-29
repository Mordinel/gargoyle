use log::error;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use super::{notify::Notify, monitor::{Monitor, Action}};

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
    /// The notifier to send if the monitor fires.
    notifier: &'a dyn Notify,
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
    pub fn add<M: Monitor, N: Notify>(
        mut self, 
        fire_message: &str,
        recover_message: &str,
        wait_time: Duration, 
        monitor: &'a M, 
        notifier: &'a N
    ) -> Schedule<'a> {
        self.entries.push(Arc::new(Mutex::new( Entry {
            fire_message: fire_message.to_string(),
            recover_message: recover_message.to_string(),
            last_checked: None,
            wait_time,
            monitor,
            notifier,
            has_fired: false,
        })));
        self
    }

    /// Run the `Schedule` instance and check all the monitors for notifications 
    /// or recoveries.
    pub fn run(&mut self) {
        rayon::scope(|s| {
            for entry in &mut self.entries {
                s.spawn(|_| {
                    let mut entry = entry.lock().unwrap();
                    handle_entry(&mut entry);
                });
            }
        });
    }
}

fn handle_entry(entry: &mut Entry) {
    // If the `Monitor` has not been checked,
    // or if the `Monitor` has not been checked in the specified amount of time,
    // check the `Monitor`.
    if entry.last_checked.is_none() || entry.last_checked.unwrap().elapsed() >= entry.wait_time {
        entry.last_checked = Some(Instant::now());
        match entry.monitor.check() {
            Action::Notify(diagnostic) => {
                if !entry.has_fired {
                    // If the `entry.notifier` fails to send the message, log the error
                    // and don't update the `has_fired` flag, this will cause the 
                    // `entry.notifier` to be sent again on the next iteration.
                    if let Err(e) = entry.notifier.send(&entry.fire_message, diagnostic) {
                        error!("{e}");
                    } else {
                        entry.has_fired = true;
                    }
                }
            }
            Action::Nothing => {
                if entry.has_fired {
                    if let Err(e) = entry.notifier.send(&entry.recover_message, None) {
                        error!("{e}");
                    } else {
                        entry.has_fired = false;
                    }
                }
            }
        }
    }
}

