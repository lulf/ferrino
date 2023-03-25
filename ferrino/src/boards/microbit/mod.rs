use embassy_executor::Spawner;
use embassy_nrf::gpio::{AnyPin, Input, Pin, Pull};

pub struct Microbit {
    pub btn_a: Input<'static, AnyPin>,
    pub btn_b: Input<'static, AnyPin>,
}

impl Microbit {
    /// Create a new instance based on HAL configuration
    pub async fn spawn(config: embassy_nrf::config::Config, _s: Spawner) -> Self {
        let p = embassy_nrf::init(config);
        Self {
            btn_a: Input::new(p.P0_14.degrade(), Pull::Up),
            btn_b: Input::new(p.P0_23.degrade(), Pull::Up),
        }
    }
}

impl crate::WithButtons for Microbit {
    type Pin = Input<'static, AnyPin>;
    fn button(&mut self) -> &mut Self::Pin {
        &mut self.btn_a
    }
}
