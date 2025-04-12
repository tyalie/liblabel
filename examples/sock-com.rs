use std::error::Error;

use futures::{AsyncReadExt, AsyncWriteExt};
use liblabel::coms::{ComSelector, PrinterCon, RFCommCon};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let selector = ComSelector::Rfcomm { mac: [0xEC, 0x79, 0x49, 0x65, 0x44, 0x2B], channel: 1 };

    let mut sock: RFCommCon = RFCommCon::open(selector).await?;
    sock.write("\x1b\x69\x61\x01".as_bytes()).await?;
    println!("wrote stuff");

    sock.write("\x1b\x69\x53".as_bytes()).await?;

    let mut buf = [0u8; 32];
    sock.read(&mut buf).await?;
    println!("buffer: {:?}", buf);
    Ok(())
}
