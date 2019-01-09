//! Serial interface echo server

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use nb::block;
use stm3240g_eval::hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let serial = stm3240g_eval::serial!(p, clocks);
    let (mut tx, mut rx) = serial.split();

    loop {
        let byte = block!(rx.read()).unwrap();
        block!(tx.write(byte)).ok();
    }
}
