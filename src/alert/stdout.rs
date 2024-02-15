use super::Alert;

#[derive(Default)]
pub struct Stdout;

impl Alert for Stdout {
    fn send(&self, msg: &str, diagnostic: Option<String>) -> Result<(), String> {
        if let Some(diagnostic) = diagnostic {
            println!("{}: {}", msg, diagnostic);
        } else {
            println!("{}", msg);
        }
        Ok(())
    }
}

