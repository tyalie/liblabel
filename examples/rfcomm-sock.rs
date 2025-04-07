use bluer::{
    rfcomm::{SocketAddr, Stream},
    Address
};
use tokio::io::AsyncWriteExt;

use std::{env, error::Error, process::exit};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Specify target Bluetooth address as argument");
        exit(1);
    }

    let target_addr: Address = args[1].parse().expect("invalid address");
    let target_sa = SocketAddr::new(target_addr, 1);

    println!("Connecting to {:?}", &target_sa);
    let mut stream = Stream::connect(target_sa).await.expect("connection failed");
    println!("Local Address: {:?}", stream.as_ref().local_addr()?);
    println!("Remote Address: {:?}", stream.peer_addr()?);
    println!("Security: {:?}", stream.as_ref().security()?);

    println!("Sending 'hello'");
    stream.write("\x1b\x69\x61\x01".as_bytes()).await?;

    Ok(())
}

