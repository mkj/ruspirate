extern crate serial_ports;
extern crate serial;

#[macro_use] extern crate failure;

mod device;
mod pirate;
pub mod i2c;
pub mod bbio;

pub use pirate::BusPirate;
pub use device::{Device, Devices};
