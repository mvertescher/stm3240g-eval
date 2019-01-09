//! Blinks the user LED using delays

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm3240g_eval::hal::{delay::Delay, prelude::*, stm32};

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    let mut delay = Delay::new(cp.SYST, clocks);

    let gpioc = p.GPIOC.split();
    let gpiog = p.GPIOG.split();
    let gpioi = p.GPIOI.split();

    let mut led1 = gpiog.pg6.into_push_pull_output();
    let mut led2 = gpiog.pg8.into_push_pull_output();
    let mut led3 = gpioi.pi9.into_push_pull_output();
    let mut led4 = gpioc.pc7.into_push_pull_output();

    loop {
        led1.set_high();
        delay.delay_ms(500_u16);
        led1.set_low();
        delay.delay_ms(500_u16);

        led2.set_high();
        delay.delay_ms(500_u16);
        led2.set_low();
        delay.delay_ms(500_u16);

        led3.set_high();
        delay.delay_ms(500_u16);
        led3.set_low();
        delay.delay_ms(500_u16);

        led4.set_high();
        delay.delay_ms(500_u16);
        led4.set_low();
        delay.delay_ms(500_u16);
    }
}
