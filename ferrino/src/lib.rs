#![no_std]

cfg_if::cfg_if! {
    if #[cfg(cortex_m)] {
        pub use ferrino_macros::main_cortex_m as main;
        pub use cortex_m_rt::entry as entry;
    }
    else if #[cfg(target_arch="riscv32")] {
        pub use ferrino_macros::main_riscv as main;
    }
    else if #[cfg(feature="wasm")] {
        pub use ferrino_macros::main_wasm as main;
    }
    else if #[cfg(feature="std")] {
        pub use ferrino_macros::main_std as main;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "board+microbit")] {
        #[path="boards/microbit/mod.rs"]
        mod board;
        pub use board::*;
        pub use board::Microbit as Board;
    } else if #[cfg(feature = "board+rpi-pico")] {
        #[path="boards/rpi-pico/mod.rs"]
        mod board;
        pub use board::*;
        pub use board::RpiPico as Board;
    }
}

pub use embassy_executor;
pub use embassy_executor::Spawner;
pub use embassy_time::*;
pub use ferrino_macros::*;

pub trait Button: Sized {
    type Pin: embedded_hal::digital::InputPin + embedded_hal_async::digital::Wait;
    fn button(&mut self) -> &mut Self::Pin;
}

pub trait Led: Sized {
    type Led: embedded_hal::digital::OutputPin;
    fn led(&mut self) -> &mut Self::Led;
}

pub trait Connectable: Sized {
    type Network: embedded_nal_async::TcpConnect;
    fn network(&mut self) -> Self::Network;
}
