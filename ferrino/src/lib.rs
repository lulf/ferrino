#![no_std]

trait Button: Sized {
    type Pin: embedded_hal::digital::InputPin + embedded_hal_async::digital::Wait;
    fn button(&mut self) -> Self::Pin;
}

pub trait Led: Sized {
    type Led: embedded_hal::digital::OutputPin;
    fn led(&mut self) -> Self::Led;
}

pub trait Connectable: Sized {
    type Network: embedded_nal_async::TcpConnect;
    fn network(&mut self) -> Self::Network;
}
