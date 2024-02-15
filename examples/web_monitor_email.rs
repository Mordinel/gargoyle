
use std::{thread::sleep, time::Duration};

use gargoyle::{
    alert,
    monitor::web,
    schedule::Schedule,
};

fn main() {
    env_logger::init();

    let smtp_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set");
    let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set");

    let smtp_from = std::env::var("SMTP_FROM").expect("SMTP_FROM not set");
    let (smtp_from_u, smtp_from_d) = smtp_from.split_once('@').expect("Invalid email address");

    let smtp_to = std::env::var("SMTP_TO").expect("SMTP_TO not set");
    let (smtp_to_u, smtp_to_d) = smtp_to.split_once('@').expect("Invalid email address");

    let smtp_relay = std::env::var("SMTP_RELAY").expect("SMTP_RELAY not set");
    let http_url = std::env::var("HTTP_URL").expect("HTTP_URL not set");

    let schedule_delay = std::env::var("SCHEDULE_DELAY_SECS")
        .expect("SCHEDULE_DELAY_SECS not set")
        .parse::<u64>()
        .expect("Invalid SCHEDULE_DELAY");

    let mail_alert = alert::Email {
        from: alert::Mailbox::new(
            Some("The Gargoyle".into()),
            alert::Address::new(smtp_from_u, smtp_from_d).expect("Invalid email address")
        ),
        to: alert::Mailbox::new(
            Some("Admin".into()),
            alert::Address::new(smtp_to_u, smtp_to_d).expect("Invalid email address")
        ),
        relay: smtp_relay,
        smtp_username,
        smtp_password,
    };

    let web_monitor = web::Availability::new(&http_url);

    let mut scheduler = Schedule::new()
        .add(
            &format!("The Gargoyle has detected that {http_url} has gone down"),
            &format!("The Gargoyle has detected that {http_url} has recovered"),
            Duration::from_secs(schedule_delay),
            &web_monitor,
            &mail_alert,
        );

    loop {
        scheduler.run();
        sleep(Duration::from_millis(100));
    }
}

