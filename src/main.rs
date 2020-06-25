#![no_main]
#![no_std]

extern crate panic_semihosting;

use stm32f1xx_hal as hal;
use hal::prelude::*;
use rtic::app;
// use cortex_m_semihosting::hprintln;
use hal::{
    time::MegaHertz,
};

const SYSCLK_FREQ: MegaHertz = MegaHertz(72);
const PCLK1_FREQ: MegaHertz = MegaHertz(36);

#[app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        foo: bool,
    }
    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        // Enable the monotonic timer
        let mut _core = cx.core;
        _core.DWT.enable_cycle_counter();

        // Set up peripherals
        let mut rcc = cx.device.RCC.constrain();
        let mut flash = cx.device.FLASH.constrain();
        let mut mapr = cx.device.AFIO.constrain(&mut rcc.apb2).mapr;
        let clocks = rcc
            .cfgr
            .sysclk(SYSCLK_FREQ)
            .pclk1(PCLK1_FREQ)
            .freeze(&mut flash.acr);

        init::LateResources {
            foo: true,
        }
    }

    extern "C" {
        fn EXTI0();
    }
};
