use core::convert::Infallible;

use embassy_executor::Spawner;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_net::{Stack as NetStack, StackResources};
use embassy_rp::gpio::{Flex, Level, Output};
use embassy_rp::interrupt;
use embassy_rp::peripherals::{PIN_23, PIN_24, PIN_25, PIN_29, USB};
use embassy_rp::usb::Driver;
use embedded_hal_async::spi::{ErrorType, ExclusiveDevice, SpiBusFlush, SpiBusRead, SpiBusWrite};
use static_cell::StaticCell;



type NetDriver = cyw43::NetDriver<'static>;

static STATE: StaticCell<cyw43::State> = StaticCell::new();
static RESOURCES: StaticCell<StackResources<4>> = StaticCell::new();
static STACK: StaticCell<NetStack<NetDriver>> = StaticCell::new();

pub struct RpiPicoW {
    control: cyw43::Control<'static>,
    stack: &'static NetStack<NetDriver>,
}

impl RpiPicoW {
    /// Create a new instance based on HAL configuration
    pub async fn spawn(config: embassy_rp::config::Config, spawner: Spawner) -> Self {
        let p = embassy_rp::init(config);

        defmt::trace!("Board init");
        let irq = interrupt::take!(USBCTRL_IRQ);
        let driver = Driver::new(p.USB, irq);
        spawner.spawn(logger_task(driver)).unwrap();

        defmt::trace!("USB spawned");
        let pwr = Output::new(p.PIN_23, Level::Low);
        let cs = Output::new(p.PIN_25, Level::High);
        let clk = Output::new(p.PIN_29, Level::Low);
        let mut dio = Flex::new(p.PIN_24);
        dio.set_low();
        dio.set_as_output();

        let bus = MySpi { clk, dio };
        let spi = ExclusiveDevice::new(bus, cs);
        defmt::trace!("SPI Cconfigured");

        let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 224190) };
        let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };

        let state = STATE.init(cyw43::State::new());
        defmt::trace!("Initializing driver");
        let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;

        defmt::trace!("Spawn wifi task");
        spawner.spawn(wifi_task(runner)).unwrap();


        control.init(clm).await;
        control
            .set_power_management(cyw43::PowerManagementMode::PowerSave)
            .await;

        let config = embassy_net::Config::Dhcp(Default::default());

        // Generate random seed
        let seed = 0x0123_4567_89ab_cdef; // chosen by fair dice roll. guarenteed to be random.

        let resources = RESOURCES.init(StackResources::new());
        let stack = STACK.init(NetStack::new(net_device, config, resources, seed));

        defmt::trace!("Spawn net task");
        spawner.spawn(net_task(stack)).unwrap();

        Self {
            control,
            stack,
        }
    }
}



#[embassy_executor::task]
async fn net_task(stack: &'static NetStack<NetDriver>) -> ! {
    stack.run().await
}

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<'static, Output<'static, PIN_23>, ExclusiveDevice<MySpi, Output<'static, PIN_25>>>,
) -> ! {
    runner.run().await
}

static CLIENT_STATE: TcpClientState<2, 1024, 1024> = TcpClientState::new();

impl crate::WithTcp for RpiPicoW {
    type Error = ();
    type TcpClient = TcpClient<'static, NetDriver, 2>;
    fn client(&mut self) -> Result<Self::TcpClient, Self::Error> {
        let client = TcpClient::new(self.stack, &CLIENT_STATE);
        Ok(client)
    }
}

impl crate::WithWifi for RpiPicoW {
    type Error = ();
    async fn join_wifi(&mut self, ssid: &str, key: &str) -> Result<(), Self::Error> {
        self.control.join_wpa2(ssid, key).await;
        Ok(())
    }
}

///////////////////////////////////////////////////////////////////////
// WIFI SPI setup
///////////////////////////////////////////////////////////////////////

struct MySpi {
    /// SPI clock
    clk: Output<'static, PIN_29>,

    /// 4 signals, all in one!!
    /// - SPI MISO
    /// - SPI MOSI
    /// - IRQ
    /// - strap to set to gSPI mode on boot.
    dio: Flex<'static, PIN_24>,
}

impl ErrorType for MySpi {
    type Error = Infallible;
}

impl SpiBusFlush for MySpi {
    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl SpiBusRead<u32> for MySpi {
    async fn read(&mut self, words: &mut [u32]) -> Result<(), Self::Error> {
        self.dio.set_as_input();
        for word in words {
            let mut w = 0;
            for _ in 0..32 {
                w = w << 1;

                // rising edge, sample data
                if self.dio.is_high() {
                    w |= 0x01;
                }
                self.clk.set_high();

                // falling edge
                self.clk.set_low();
            }
            *word = w
        }

        Ok(())
    }
}

impl SpiBusWrite<u32> for MySpi {
    async fn write(&mut self, words: &[u32]) -> Result<(), Self::Error> {
        self.dio.set_as_output();
        for word in words {
            let mut word = *word;
            for _ in 0..32 {
                // falling edge, setup data
                self.clk.set_low();
                if word & 0x8000_0000 == 0 {
                    self.dio.set_low();
                } else {
                    self.dio.set_high();
                }

                // rising edge
                self.clk.set_high();

                word = word << 1;
            }
        }
        self.clk.set_low();

        self.dio.set_as_input();
        Ok(())
    }
}
