use crate::notify::Notify;

#[derive(Default)]
pub struct Stdout;

/// Send a notification to stdout
impl Notify for Stdout {
    fn send(&self, msg: &str, diagnostic: Option<String>) -> Result<(), String> {
        if let Some(diagnostic) = diagnostic {
            println!("{}: {}", msg, diagnostic);
        } else {
            println!("{}", msg);
        }
        Ok(())
    }
}

