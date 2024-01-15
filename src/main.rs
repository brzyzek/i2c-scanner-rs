// I2C Scanner written in Rust for an STM32F446 processor.

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use core::fmt::Write;

use crate::hal::{
    prelude::*,
    pac
};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Clock Config
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(25.MHz()).freeze();

        let mut delay = cp.SYST.delay(&clocks);

        // LED Pin Config
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        // UART Pin Config
        let uart_tx_pin = gpioa.pa2;
        let mut uart_tx = dp.USART2.tx(uart_tx_pin, 115200.bps(), &clocks).unwrap();

        let mut value: u8 = 0;

        loop {
            writeln!(uart_tx, "value: {value:02}\r").unwrap();
            value = value.wrapping_add(1);
            delay.delay(2.secs());
            led.toggle();
            delay.delay_ms(100_u32);

        }
    }

    loop {}
}