#![allow(dead_code)]
pub mod gpio;
pub mod clock;
pub mod delay;
pub mod led;
pub mod hwlcd;

use stm32f103xx::{GPIOB, GPIOC};

pub const FREQUENCY: u32 = 72_000_000;

// LCD
pub const RS: gpio::PinRange<GPIOB> = gpio::PinRange::new(12, 1);
pub const RW: gpio::PinRange<GPIOB> = gpio::PinRange::new(13, 1);
pub const E: gpio::PinRange<GPIOB> = gpio::PinRange::new(14, 1);
pub const DATA: gpio::PinRange<GPIOB> = gpio::PinRange::new(6, 4);

// LED
pub const LED: gpio::PinRange<GPIOC> = gpio::PinRange::new(13, 1);