use stm32f103xx::{GPIOC, RCC};

pub struct Led {}

impl Led {
    pub const fn new() -> Led {
        Led {}
    }

    pub fn init(&self, gpioc: &GPIOC, rcc: &RCC) {
        rcc.apb2enr.modify(|_, w| w.iopcen().enabled());

        // PC13 is LED
        gpioc.crh.modify(|_, w| w.cnf13().open().mode13().output2());
        ::hw::LED.set(gpioc, 1); // Turn it off
    }

    pub fn set<'a>(&self, gpioc: &'a GPIOC, on: bool) {
        ::hw::LED.set(gpioc, if on { 0 } else { 1 }); // '0' is on
    }
}
