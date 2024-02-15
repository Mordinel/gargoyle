use std::{thread::sleep, time::Duration};

use gargoyle::{
    alert,
    monitor::{local, web},
    schedule::Schedule,
};

fn main() {
    env_logger::init();

    // Create a new `Stdout` instance of `Alert`.
    let stdout_alert = alert::Stdout;

    // Create a new `local::ExactService` instance of `Monitor`.
    let top_monitor = local::ExactService {
        process_name: "top".to_string(),
    };

    // Create a new `web::Availability` instance of `Monitor`.
    let web_monitor = web::Availability::new("http://127.0.0.1:9001/index.html");

    // Create a new `Schedule` instance and add the monitors and alerts to it.
    let mut scheduler = Schedule::new()
        .add("`top` has gone down",
             "`top` has recovered",
             Duration::from_secs(5),
             &top_monitor,
             &stdout_alert
        )
        .add("`web service` has gone down",
             "`web service` has recovered",
             Duration::from_secs(10),
             &web_monitor,
             &stdout_alert
        );
    
    loop {
        scheduler.run();
        sleep(Duration::from_millis(100));
    }
}

