use embassy_rp::gpio::{AnyPin, Level, Output, Pin};

pub struct RpiPico {
    pub led: Output<'static, AnyPin>,
}

impl Default for RpiPico {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl RpiPico {
    /// Create a new instance based on HAL configuration
    pub fn new(config: embassy_rp::config::Config) -> Self {
        let p = embassy_rp::init(config);
        Self {
            led: Output::new(p.PIN_25.degrade(), Level::Low),
        }
    }
}

impl crate::Led for RpiPico {
    type Led = Output<'static, AnyPin>;
    fn led(&mut self) -> &mut Self::Led {
        &mut self.led
    }
}
