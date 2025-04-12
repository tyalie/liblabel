#[cfg(feature = "rfcomm")]
pub mod rfcomm;

pub mod interface;

#[cfg(feature = "rfcomm")]
pub use rfcomm::RFCommCon;

pub use interface::{ComSelector, PrinterCon, ComError};
