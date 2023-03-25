#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

use ferrino::*;

use embedded_nal_async::{SocketAddr, TcpConnect};

use defmt_rtt as _;
use panic_reset as _;

const WIFI_SSID: &str = ""; //include_str!("wifi.ssid");
const WIFI_PSK: &str = ""; //include_str!("wifi.psk");

#[ferrino::main]
async fn main(mut board: impl WithTcp + WithWifi, _spawner: Spawner) {
    defmt::info!("Joining wifi...");
    let _ = board.join_wifi(WIFI_SSID.trim_end(), WIFI_PSK.trim_end()).await.unwrap();
    defmt::info!("Wifi joined! Connecting to host...");
    let client = board.client().unwrap();
    let connection = client.connect("127.0.0.1:8080".parse().unwrap()).await.unwrap();
    defmt::info!("Connected!");

    loop {
        Timer::after(Duration::from_secs(60)).await;
    }
}
