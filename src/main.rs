#![no_main]
#![no_std]

use rtt_target::{rtt_init_print, rprintln};
use core::panic::PanicInfo;
use stm32f1xx_hal as hal;
use hal::prelude::*;
use rtic::app;
use hal::{
    time::MegaHertz,
};

const FREQ: u32 = 48;
const SYSCLK_FREQ: MegaHertz = MegaHertz(FREQ);
const PCLK1_FREQ: MegaHertz = MegaHertz(FREQ / 2);

#[app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        foo: bool,
    }
    #[init]
    fn init(cx: init::Context) -> init::LateResources {

        // Initialize RTT
        rtt_init_print!();

        // Enable the monotonic timer
        let mut _core = cx.core;
        _core.DWT.enable_cycle_counter();

        // Set up peripherals
        let mut rcc = cx.device.RCC.constrain();
        let mut flash = cx.device.FLASH.constrain();
        let mut _mapr = cx.device.AFIO.constrain(&mut rcc.apb2).mapr;
        let _clocks = rcc
            .cfgr
            .sysclk(SYSCLK_FREQ)
            .pclk1(PCLK1_FREQ)
            .freeze(&mut flash.acr);

        rprintln!("Hello world.");

        init::LateResources {
            foo: true,
        }
    }

    // We need an idle to prevent it from going to sleep
    #[idle]
    fn idle(_: idle::Context) -> ! {
        // prevents WFI which would disable RTT
        loop { }
    }

    extern "C" {
        fn EXTI0();
    }
};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {}
}