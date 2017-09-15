#![feature(const_fn)]
#![feature(used)]
#![feature(proc_macro)]
#![no_std]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#![cfg_attr(feature = "cargo-clippy", allow(eq_op))]
//#![deny(warnings)]

extern crate stm32f103xx;
extern crate cortex_m_rtfm as rtfm;

use hw::{delay, clock, led};
use rtfm::{app, Threshold};

mod hw;

static LED: led::Led = led::Led::new();

app! {
    device: stm32f103xx,

    idle: {
        resources: [SYST, GPIOC],
    },
}

fn init(p: init::Peripherals) {
    clock::setup(p.RCC, p.SYST, p.FLASH);

    // Initialize hardware
    LED.init(p.GPIOC, p.RCC);
}

fn idle(_t: &mut Threshold, r: idle::Resources) -> ! {
    loop {
        LED.set(r.GPIOC, true);
        delay::ms(r.SYST, 500);
        LED.set(r.GPIOC, false);
        delay::ms(r.SYST, 500);
    }
}
