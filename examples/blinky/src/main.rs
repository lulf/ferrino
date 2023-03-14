#![feature(type_alias_impl_trait)]
#![no_std]
#![no_main]

use ferrino::*;

use embedded_hal::digital::OutputPin;

use panic_reset as _;
use defmt_rtt as _;

#[ferrino::main]
async fn main(mut board: impl Led, _spawner: Spawner)
{
    loop {
        let _ = board.led().set_high();
        Timer::after(Duration::from_secs(1)).await;
        let _ = board.led().set_low();
        Timer::after(Duration::from_secs(1)).await;
    }
}
