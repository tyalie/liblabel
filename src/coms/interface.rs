use futures::{AsyncRead, AsyncWrite};
use snafu::Snafu;

use std::net::SocketAddr;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ComError {
    #[cfg(feature = "rfcomm")]
    #[snafu(display("Unnamed bluez error"))]
    BluezError { source: bluer::Error },

    #[snafu(display("Connection error"))]
    ConnectionError { source: std::io::Error },

    #[snafu(display("Expected selector(s) {}, but got {:?}", expected, got))]
    IncompatibleSelectorError { expected: String, got: ComSelector },
}

#[non_exhaustive]
#[derive(Debug)]
pub enum ComSelector {
    Tcp(SocketAddr),

    #[cfg(feature = "usb")]
    Usb {
        vid: u16,
        pid: u16,
    },
    #[cfg(feature = "rfcomm")]
    Rfcomm {
        mac: [u8; 6],
        channel: u8,
    },
}

pub struct ComDevice {
    name: String,
    selector: ComSelector,
}

pub trait PrinterCon: Sized {
    fn open(selector: ComSelector) -> impl Future<Output = Result<Self, ComError>>;
    fn discover() -> impl Future<Output = Result<Vec<ComDevice>, ComError>>;

    fn take_stream(&mut self) -> Option<impl AsyncWrite + AsyncRead + Send + 'static>;
}
