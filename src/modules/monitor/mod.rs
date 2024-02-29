#[cfg(feature = "local-monitor")]
mod local;

#[cfg(feature = "local-monitor")]
#[cfg_attr(docsrs, doc(cfg(feature = "local-monitor")))]
pub use local::*;

#[cfg(feature = "web-monitor")]
mod web;

#[cfg(feature = "web-monitor")]
#[cfg_attr(docsrs, doc(cfg(feature = "web-monitor")))]
pub use web::*;

