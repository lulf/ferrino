use embassy_nrf::gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull};

pub struct Microbit {
    pub btn_a: Input<'static, AnyPin>,
    pub btn_b: Input<'static, AnyPin>,
}
impl Default for Microbit {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl Microbit {
    /// Create a new instance based on HAL configuration
    pub fn new(config: embassy_nrf::config::Config) -> Self {
        let p = embassy_nrf::init(config);
        Self {
            btn_a: Input::new(p.P0_14.degrade(), Pull::Up),
            btn_b: Input::new(p.P0_23.degrade(), Pull::Up),
        }
    }
}

impl crate::Button for Microbit {
    type Pin = Input<'static, AnyPin>;
    fn button(&mut self) -> &mut Self::Pin {
        &mut self.btn_a
    }
}
