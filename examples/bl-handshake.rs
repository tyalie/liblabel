use std::error::Error;
use std::str::FromStr;
use tokio;

use btleplug::api::{Manager as _, Peripheral, Central, BDAddr};
use btleplug::platform::Manager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;

    println!("Adapter list:");
    for adapter in adapter_list.iter() {
        println!("- {}", adapter.adapter_info().await?);
        let peripherals = adapter.peripherals().await?;
        for peripheral in peripherals.iter() {
            let properties = peripheral.properties().await?.unwrap();
            let is_connected = peripheral.is_connected().await?;
            let mac = properties.address;
            let local_name = properties
                .local_name
                .unwrap_or(String::from("(peripheral name unknown)"));
            println!("Peripheral ({:?}) {:?} - is_connected: {:?}", mac, local_name, is_connected);

            if mac == BDAddr::from_str("EC:79:49:65:44:2B").unwrap() {
                println!("- matched");

                if !is_connected {
                    println!("- connecting");
                    if let Err(err) = peripheral.connect().await {
                        eprintln!("Error connecting to peripheral, skipping {}", err);
                        continue;
                    }
                }

                peripheral.discover_services().await?;
                println!("Discover peripheral {:?} services...", &local_name);

                for service in peripheral.services() {
                    println!(
                        "Service UUID {}, primary: {}",
                        service.uuid, service.primary
                    );
                    for characteristic in service.characteristics {
                        println!("  {:?}", characteristic);
                    }
                }
            }

        }
    }

    Ok(())
}
