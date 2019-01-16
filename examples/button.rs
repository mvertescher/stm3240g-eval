//! Test out the push buttons!
//!
//! - Wakeup push button: A0
//! - Tamper push button: C13
//! - Key push button:    G15

// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_semihosting;

use core::{cell::RefCell, ops::DerefMut};

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use stm3240g_eval::hal::{delay::Delay, interrupt, prelude::*, stm32, stm32::Interrupt};
use stm32f4xx_hal::stm32::{EXTI, SYSCFG};

// Make external interrupt registers globally available
static INT: Mutex<RefCell<Option<EXTI>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    let mut delay = Delay::new(cp.SYST, clocks);

    let exti = p.EXTI;
    let syscfg = p.SYSCFG;

    let gpioa = p.GPIOA.split();
    let gpioc = p.GPIOC.split();
    let gpiog = p.GPIOG.split();

    // Configure as input (button)
    let _ = gpioa.pa0.into_pull_down_input();
    let _ = gpioc.pc13.into_pull_down_input();
    let _ = gpiog.pg15.into_pull_down_input();

    configure_pa0(&syscfg, &exti);
    // configure_pc13(&syscfg, &exti);
    configure_pg15(&syscfg, &exti);

    // Move control over to the global mutex
    cortex_m::interrupt::free(move |cs| {
        *INT.borrow(cs).borrow_mut() = Some(exti);
    });

    let mut nvic = cp.NVIC;
    nvic.enable(Interrupt::EXTI0);
    nvic.enable(Interrupt::EXTI15_10);
    unsafe { nvic.set_priority(Interrupt::EXTI0, 1) };
    unsafe { nvic.set_priority(Interrupt::EXTI15_10, 1) };
    cortex_m::peripheral::NVIC::unpend(Interrupt::EXTI0);
    cortex_m::peripheral::NVIC::unpend(Interrupt::EXTI15_10);

    loop {
        cortex_m_semihosting::hprintln!(".").unwrap();
        delay.delay_ms(1_000_u16);
    }
}

fn configure_pa0(syscfg: &SYSCFG, exti: &EXTI) {
    // Enable external interrupt for PA0
    syscfg.exticr1.modify(|_, w| unsafe { w.exti1().bits(0) });

    // Set interrupt request mask for line 0
    exti.imr.modify(|_, w| w.mr0().set_bit());

    // Set interrupt rising trigger for line 0
    exti.rtsr.modify(|_, w| w.tr0().set_bit());
}

fn configure_pc13(syscfg: &SYSCFG, exti: &EXTI) {
    syscfg.exticr1.modify(|_, w| unsafe { w.exti1().bits(2) });
    exti.imr.modify(|_, w| w.mr13().set_bit());
    exti.rtsr.modify(|_, w| w.tr13().set_bit());
}

fn configure_pg15(syscfg: &SYSCFG, exti: &EXTI) {
    syscfg.exticr1.modify(|_, w| unsafe { w.exti1().bits(6) });
    exti.imr.modify(|_, w| w.mr15().set_bit());
    exti.rtsr.modify(|_, w| w.tr15().set_bit());
}

interrupt!(EXTI0, exti0);

fn exti0() {
    // Enter critical section
    cortex_m::interrupt::free(|cs| {
        if let &mut Some(ref mut exti) = INT.borrow(cs).borrow_mut().deref_mut() {
            cortex_m_semihosting::hprintln!("wakeup").unwrap();
            // Clear interrupt
            exti.pr.write(|w| w.pr0().set_bit());
        }
    });
}

interrupt!(EXTI15_10, exti15_10);

fn exti15_10() {
    cortex_m::interrupt::free(|cs| {
        if let &mut Some(ref mut exti) = INT.borrow(cs).borrow_mut().deref_mut() {
            if exti.pr.read().pr13().bit_is_set() {
                cortex_m_semihosting::hprintln!("tamper").unwrap();
                exti.pr.write(|w| w.pr13().set_bit());
            }

            if exti.pr.read().pr15().bit_is_set() {
                cortex_m_semihosting::hprintln!("key").unwrap();
                exti.pr.write(|w| w.pr15().set_bit());
            }
        }
    });
}
