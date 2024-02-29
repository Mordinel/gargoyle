#[cfg(feature = "email-notifier")]
mod email;

#[cfg(feature = "email-notifier")]
pub use email::*;

mod stdout;
pub use stdout::*;
