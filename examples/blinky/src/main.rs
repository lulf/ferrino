#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

use defmt::*;
use ferrino::*;

use panic_reset as _;
use defmt_rtt as _;

#[ferrino::main]
async fn main(mut device: Device, _spawner: Spawner) {
    loop {
        device.button().wait_for_any_edge().await;
        info!("Edge triggered!");
    }
}
