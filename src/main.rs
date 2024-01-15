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
    pac,
    i2c::Mode
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

        // LED Config
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        // UART Config
        let uart_tx_pin = gpioa.pa2;
        let mut uart_tx = dp.USART2.tx(uart_tx_pin, 115200.bps(), &clocks).unwrap();

        // I2C Config
        let gpiob = dp.GPIOB.split();
        let i2c_scl_pin = gpiob.pb8;
        let i2c_sda_pin = gpiob.pb9;

        

        let mut i2c = dp.I2C1.i2c(
            (i2c_scl_pin, i2c_sda_pin),
            Mode::Standard {
                frequency: 100.kHz(),
            },
            &clocks,
        );
    
        let mut buffer: [u8; 255] = [0; 255];

        loop {
            writeln!(uart_tx, "\r").unwrap();
            writeln!(uart_tx, "Starting I2C Scanner... (If addresses do not show up, there is no device on the I2C bus.)\r").unwrap();
            led.toggle();
            for address in 0..127 {
                let i2c_result = i2c.read(address, &mut buffer);
                match i2c_result {
                    Ok(_) => write!(uart_tx, " {address:#02x} ").unwrap(),
                    Err(_) => write!(uart_tx, "  ~~  ").unwrap(),
            }

        }
            writeln!(uart_tx, "\r").unwrap();
            writeln!(uart_tx, "Finished.\r").unwrap();
            led.toggle();
            delay.delay_ms(60000_u32);

        }
    }

    loop {}
    }
