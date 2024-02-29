use std::{thread::sleep, time::Duration};

use gargoyle::{
    Schedule,
    modules::{notify, monitor},
};

fn main() {
    env_logger::init();

    // Create a new `Stdout` instance of `Notifier`.
    let stdout_notifier = notify::Stdout::default();

    // Create a new `ExactService` instance of `Monitor`.
    let mut top_monitor = monitor::ExactService::new("top");

    // Create a new `WebAvailability` instance of `Monitor`.
    let mut web_monitor = monitor::WebAvailability::new("http://127.0.0.1:9001/index.html");

    // Create a new `Schedule` instance and add the monitors and alerts to it.
    let mut scheduler = Schedule::default();
    scheduler.add(
        "`top` has gone down",
        "`top` has recovered",
        Duration::from_secs(5),
        &mut top_monitor,
        &stdout_notifier
    );
    scheduler.add(
        "`web service` has gone down",
        "`web service` has recovered",
        Duration::from_secs(10),
        &mut web_monitor,
        &stdout_notifier
    );
    
    loop {
        scheduler.run();
        sleep(Duration::from_millis(100));
    }
}

