use embassy_executor::Spawner;
use embassy_rp::gpio::{AnyPin, Level, Output, Pin};

pub struct RpiPico {
    pub led: Output<'static, AnyPin>,
}

impl RpiPico {
    /// Create a new instance based on HAL configuration
    pub fn spawn(config: embassy_rp::config::Config, _s: Spawner) -> Self {
        let p = embassy_rp::init(config);
        Self {
            led: Output::new(p.PIN_25.degrade(), Level::Low),
        }
    }
}

impl crate::WithLeds for RpiPico {
    type Led = Output<'static, AnyPin>;
    fn led(&mut self) -> &mut Self::Led {
        &mut self.led
    }
}
