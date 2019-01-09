//! Board Support Crate for the STM3240G-EVAL

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

pub extern crate stm32f4xx_hal as hal;

/// Configure the default serial port with the default configuration.
///
/// The baud rate of the serial port should be 19200.
#[macro_export]
macro_rules! serial {
    ($p:ident, $clocks:ident) => {{
        let mut gpioc = $p.GPIOC.split();
        let tx = gpioc.pc10.into_alternate_af7();
        let rx = gpioc.pc11.into_alternate_af7();

        let config = stm3240g_eval::hal::serial::config::Config::default();
        stm3240g_eval::hal::serial::Serial::usart3($p.USART3, (tx, rx), config, $clocks).unwrap()
    }};
}

pub mod led;
