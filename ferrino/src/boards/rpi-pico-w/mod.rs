use core::convert::Infallible;
use core::future::Future;
use core::mem::MaybeUninit;
use embassy_executor::{Executor, Spawner};
use embassy_net::{
    tcp::client::{TcpClient, TcpClientState},
    Stack as NetStack, StackResources,
};
use embassy_rp::gpio::{AnyPin, Flex, Level, Output, Pin};
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_rp::peripherals::{PIN_23, PIN_24, PIN_25, PIN_29};
use embedded_hal_async::spi::{ErrorType, ExclusiveDevice, SpiBusFlush, SpiBusRead, SpiBusWrite};
use static_cell::StaticCell;

pub struct RpiPicoW {
    wifi: Option<WifiPeripheral>,
}

struct WifiPeripheral {
    pwr: Output<'static, PIN_23>,
    spi: ExclusiveDevice<MySpi, Output<'static, PIN_25>>,
}

impl Default for RpiPicoW {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

type NetDriver = cyw43::NetDriver<'static>;

impl RpiPicoW {
    /// Create a new instance based on HAL configuration
    pub fn new(config: embassy_rp::config::Config) -> Self {
        let p = embassy_rp::init(config);

        let pwr = Output::new(p.PIN_23, Level::Low);
        let cs = Output::new(p.PIN_25, Level::High);
        let clk = Output::new(p.PIN_29, Level::Low);
        let mut dio = Flex::new(p.PIN_24);
        dio.set_low();
        dio.set_as_output();

        let bus = MySpi { clk, dio };
        let spi = ExclusiveDevice::new(bus, cs);
        Self {
            wifi: Some(WifiPeripheral { spi, pwr }),
        }
    }

    pub fn spawn(&mut self, spawner: Spawner) {
        let peri = self.wifi.take().unwrap();

        spawner.spawn(system(peri, spawner));
    }
}

static STATE: StaticCell<cyw43::State> = StaticCell::new();
static RESOURCES: StaticCell<StackResources<4>> = StaticCell::new();
static mut STACK: MaybeUninit<NetStack<NetDriver>> = MaybeUninit::uninit();
static mut CONTROL: MaybeUninit<cyw43::Control<'static>> = MaybeUninit::uninit();

#[embassy_executor::task]
async fn system(peri: WifiPeripheral, spawner: Spawner) {
    let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 224190) };
    let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };

    let state = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, peri.pwr, peri.spi, fw).await;

    spawner.spawn(wifi_task(runner)).unwrap();

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    unsafe { CONTROL.write(control) };

    let config = embassy_net::Config::Dhcp(Default::default());

    // Generate random seed
    let seed = 0x0123_4567_89ab_cdef; // chosen by fair dice roll. guarenteed to be random.

    let resources = RESOURCES.init(StackResources::new());

    let stack = unsafe { STACK.write(NetStack::new(net_device, config, resources, seed)) };

    stack.run().await
}

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<
        'static,
        Output<'static, PIN_23>,
        ExclusiveDevice<MySpi, Output<'static, PIN_25>>,
    >,
) -> ! {
    runner.run().await
}

static CLIENT_STATE: TcpClientState<2, 1024, 1024> = TcpClientState::new();

impl crate::WithTcp for RpiPicoW {
    type Error = ();
    type TcpClient = TcpClient<'static, NetDriver, 2>;
    fn client(&mut self) -> Result<Self::TcpClient, Self::Error> {
        let stack = unsafe { STACK.assume_init_ref() };
        let client = TcpClient::new(stack, &CLIENT_STATE);
        Ok(client)
    }
}

impl crate::WithWifi for RpiPicoW {
    type Error = ();
    async fn join_wifi(&mut self, ssid: &str, key: &str) -> Result<(), Self::Error> {
        let control = unsafe { CONTROL.assume_init_mut() };
        control.join_wpa2(ssid, key).await;
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
