#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use cortex_m::asm;

#[rtic::app(device = stm32f1::stm32f103, peripherals = true)]
const APP: () = {
    #[init]
    fn init(cx: init::Context) {
        let _core: cortex_m::Peripherals = cx.core;
        let device: stm32f1::stm32f103::Peripherals = cx.device;

        device.RCC.apb2enr.modify(|_, w| w.iopcen().enabled());
        device.GPIOC.crh.modify(|_, w| w.cnf13().open_drain().mode13().output2());
        device.GPIOC.bsrr.write(|w| w.br13().set_bit());
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            asm::wfi();
        }
    }
};