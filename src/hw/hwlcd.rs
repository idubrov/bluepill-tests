extern crate lcd;

use stm32f103xx::{GPIOB, SYST, RCC};

/// Wrapper type to create HD44780 instances as needed
pub struct Lcd {}

impl Lcd {
    pub const fn new() -> Lcd {
        Lcd {}
    }

    pub fn init(&self, gpiob: &GPIOB, rcc: &RCC) {
        rcc.apb2enr.modify(|_, w| w.iopben().enabled());
        gpiob.crl.modify(|_, w| w
            // PB6-PB7 are DB4-DB5
            .cnf6().push().mode6().output()
            .cnf7().push().mode7().output());
        gpiob.crh.modify(|_, w| w
            // PB12 is RS
            .cnf12().push().mode12().output()
            // PB13 is R/W
            .cnf13().push().mode13().output()
            // PB14 is E
            .cnf14().push().mode14().output()
            // PB8-PB9 are DB6-DB7
            .cnf8().push().mode8().output()
            .cnf9().push().mode9().output());

        // R/W is always 0 -- we don't use wait flag
        ::hw::RW.set(gpiob, 0);
    }

    pub fn materialize<'a>(&self, syst: &'a SYST, gpiob: &'a GPIOB) -> lcd::Display<LcdHw<'a>> {
        lcd::Display::new(LcdHw {
            syst,
            gpiob,
        })
    }
}

/// Binding of HD44780 instance to real hardware
pub struct LcdHw<'a> {
    syst: &'a SYST,
    gpiob: &'a GPIOB,
}

impl<'a> lcd::Hardware for LcdHw<'a> {
    fn rs(&self, bit: bool) {
        ::hw::RS.set(self.gpiob, if bit { 1 } else { 0 });
    }

    fn enable(&self, bit: bool) {
        ::hw::E.set(self.gpiob, if bit { 1 } else { 0 });
    }

    fn data(&self, data: u8) {
        ::hw::DATA.set(self.gpiob, u16::from(data));
    }
}

impl<'a> lcd::Delay for LcdHw<'a> {
    fn delay_us(&self, delay_usec: u32) {
        ::hw::delay::us(self.syst, delay_usec);
    }
}

impl<'a> lcd::InputCapableHardware for LcdHw<'a> {
    fn rw(&self, bit: bool) {
        if bit {
            // LCD has OD output, set all to '0' just to be sure.
            ::hw::DATA.set(self.gpiob, 0);

            // open == 01 == floating input
            self.gpiob.crl.modify(|_, w| w
                .cnf6().open().mode6().input()
                .cnf7().open().mode7().input()
            );
            self.gpiob.crh.modify(|_, w| w
                .cnf8().open().mode8().input()
                .cnf9().open().mode9().input());

            // Finally, set R/W to 1 (read)
            ::hw::RW.set(self.gpiob, 1);
        } else {
            // First, set R/W to 0 (write mode)
            ::hw::RW.set(self.gpiob, 0);

            // To be sure LCD is in read mode
            ::hw::delay::us(self.syst, 1);

            // Reset data port to output
            self.gpiob.crl.modify(|_, w| w
                .cnf6().push().mode6().output()
                .cnf7().push().mode7().output());
            self.gpiob.crh.modify(|_, w| w
                .cnf8().push().mode8().output()
                .cnf9().push().mode9().output());
        }
    }

    fn read_data(&self) -> u8 {
        ::hw::DATA.get(self.gpiob) as u8
    }
}