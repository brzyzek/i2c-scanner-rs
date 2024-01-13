#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let gpioa = dp.GPIOA.split();
        let gpioc = dp.GPIOC.split();
        let mut led = gpioa.pa5.into_push_pull_output();
        let button = gpioc.pc13.into_pull_up_input();

        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        let mut delay = cp.SYST.delay(&clocks);

        let mut state = false;

        loop {
            if button.is_low(){ 
                state = true;
                delay.delay_ms(50_u32);
            }
            
            if button.is_high() && state == true{
                led.toggle();
                state = false;
            }
        }
    }

    loop {}
}