use super::Alert;

pub struct Stdout;

impl Alert for Stdout {
    fn send(&self, msg: &str) -> Result<(), String> {
        println!("{}", msg);
        Ok(())
    }
}

