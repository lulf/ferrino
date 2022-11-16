use defmt::*;
use ferrino::*;

#[derive(Button)]
pub struct Device;

#[ferrino::main]
async fn main(device: Device) {
    loop {
        device.button().wait_for_any_edge().await;
        info!("Edge triggered!");
    }
}
