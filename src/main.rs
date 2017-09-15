#![feature(const_fn)]
#![feature(used)]
#![feature(proc_macro)]
#![no_std]
#![cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#![cfg_attr(feature = "cargo-clippy", allow(eq_op))]

extern crate stm32f103xx;
extern crate cortex_m_rtfm as rtfm;
extern crate lcd;

use hw::{delay, clock, led, hwlcd};
use rtfm::{app, Threshold};
use core::fmt::Write;
use stm32f103xx::{SYST, GPIOB};

mod hw;

static LCD: hwlcd::Lcd = hwlcd::Lcd::new();
static LED: led::Led = led::Led::new();

app! {
    device: stm32f103xx,

    idle: {
        resources: [SYST, GPIOB, GPIOC],
    },
}

fn init_screen(syst: &SYST, gpiob: &GPIOB) {
    let mut lcd = LCD.materialize(syst, gpiob);
    lcd.init(lcd::FunctionLine::Line2, lcd::FunctionDots::Dots5x8);
    lcd.display(lcd::DisplayMode::DisplayOn, lcd::DisplayCursor::CursorOff, lcd::DisplayBlink::BlinkOff);
    //font::upload_characters(&mut lcd);
    lcd.entry_mode(lcd::EntryModeDirection::EntryRight, lcd::EntryModeShift::NoShift);
}

fn init(p: init::Peripherals) {
    clock::setup(p.RCC, p.SYST, p.FLASH);

    // Initialize hardware
    LED.init(p.GPIOC, p.RCC);
    LCD.init(p.GPIOB, p.RCC);

    // Need to wait at least 40ms after Vcc rises to 2.7V
    // STM32 could start much earlier than that
    delay::ms(p.SYST, 50);
    init_screen(p.SYST, p.GPIOB);
}

fn idle(_t: &mut Threshold, r: idle::Resources) -> ! {
    let mut lcd = LCD.materialize(r.SYST, r.GPIOB);
    let mut delay = 0;
    loop {
        r.SYST.clear_current();
        lcd.position(0, 0);
        write!(&mut lcd, "{: >3}     ", delay).unwrap();

        delay = r.SYST.get_current();
    }
}
