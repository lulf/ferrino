#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

use ferrino::*;

use embedded_nal_async::{SocketAddr, TcpConnect};

use defmt_rtt as _;
use panic_reset as _;

#[ferrino::main]
async fn main(mut board: impl WithTcp, _spawner: Spawner) {
    let client = board.client();
    let connection = client.connect("127.0.0.1:8080".parse().unwrap()).await;

    loop {
        Timer::after(Duration::from_secs(60)).await;
    }
}
