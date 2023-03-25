#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

use ferrino::*;

use embedded_nal_async::{TcpConnect};
use embedded_io::asynch::{Write, Read};

use defmt_rtt as _;
//use panic_reset as _;
use panic_probe as _;

const WIFI_SSID: &str = ""; //include_str!("wifi.ssid");
const WIFI_PSK: &str = ""; //include_str!("wifi.psk");

#[ferrino::main]
async fn main(mut board: impl WithTcp + WithWifi, _spawner: Spawner) {
    defmt::info!("Application started");
    Timer::after(Duration::from_secs(10)).await;
    defmt::info!("Joining wifi...");
    let _ = board.join_wifi(WIFI_SSID.trim_end(), WIFI_PSK.trim_end()).await.unwrap();

    defmt::info!("Wifi joined! Connecting to host...");
    let client = board.client().unwrap();

    let mut connection = client.connect("127.0.0.1:8080".parse().unwrap()).await.unwrap();
    defmt::info!("Connected!");

    loop {
        let mut rx = [0; 4];
        defmt::info!("Sending ping");
        connection.write_all(b"PING").await.unwrap();
        let sz = connection.read(&mut rx[..]).await.unwrap();
        defmt::info!("Received: {:?}", &rx[..sz]);

        Timer::after(Duration::from_secs(60)).await;
    }
}
