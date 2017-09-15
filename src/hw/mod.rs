#![allow(dead_code)]
pub mod gpio;
pub mod clock;
pub mod delay;
pub mod led;

use stm32f103xx::{GPIOC};

pub const FREQUENCY: u32 = 72_000_000;

// LED
pub const LED: gpio::PinRange<GPIOC> = gpio::PinRange::new(13, 1);