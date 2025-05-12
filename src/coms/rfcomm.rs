use std::pin::pin;

use super::interface::{ComDevice, ComError, ComSelector, ConnectionSnafu, PrinterCon};
use futures::{AsyncRead, AsyncWrite};

use bluer::{
    Address,
    rfcomm::{SocketAddr, Stream},
};
use pin_project::pin_project;
use snafu::ResultExt;
use std::task::Poll;
use tokio::io::{AsyncRead as _, AsyncWrite as _, ReadBuf};

pub struct RFCommCon {
    stream: Option<BlueZStreamWrapper>,
}

#[pin_project]
struct BlueZStreamWrapper {
    #[pin]
    stream: bluer::rfcomm::Stream,
}

impl AsyncRead for BlueZStreamWrapper {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        let mut readbuf = ReadBuf::new(buf);

        match self.project().stream.poll_read(cx, &mut readbuf) {
            Poll::Ready(_) => Poll::Ready(Ok(readbuf.capacity() - readbuf.remaining())),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl AsyncWrite for BlueZStreamWrapper {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        self.project().stream.poll_write(cx, buf)
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        self.project().stream.poll_flush(cx)
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        self.project().stream.poll_shutdown(cx)
    }
}

impl PrinterCon for RFCommCon {
    fn open(selector: ComSelector) -> impl Future<Output = Result<Self, ComError>> {
        async move {
            match selector {
                ComSelector::Rfcomm { mac, channel } => {
                    let address = Address::new(mac);
                    let target_addr = SocketAddr::new(address, channel);

                    let stream = Stream::connect(target_addr)
                        .await
                        .context(ConnectionSnafu {})?;

                    Ok(Self {
                        stream: Some(BlueZStreamWrapper { stream }),
                    })
                }
                other => Err(ComError::IncompatibleSelectorError {
                    expected: "ComSelector::rfcomm".into(),
                    got: other,
                }),
            }
        }
    }

    fn discover() -> impl Future<Output = Result<Vec<ComDevice>, ComError>> {
        async move { Ok(vec![]) }
    }

    fn take_stream(&mut self) -> Option<impl AsyncWrite + AsyncRead + Send + 'static> {
        self.stream.take()
    }
}
