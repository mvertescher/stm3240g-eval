//! Test out the push buttons!
//!
//! - Wakeup push button: A0
//! - Tamper push button: C13
//! - Key push button:    G15

// #![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;
extern crate cortex_m_rt;

use core::{cell::RefCell, ops::DerefMut};

use cortex_m_rt::entry;
use stm3240g_eval::hal::{delay::Delay, prelude::*, stm32, interrupt, stm32::Interrupt};
use stm32f4xx_hal::stm32::EXTI;
use cortex_m::interrupt::Mutex;

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

    let gpioc = p.GPIOC.split();

	// Configure PC13 as input (button)
	let _ = gpioc.pc13.into_pull_down_input();

	// Enable external interrupt for PC0
	syscfg
		.exticr1
		.modify(|_, w| unsafe { w.exti1().bits(0) });

	// Set interrupt request mask for line 0
	exti.imr.modify(|_, w| w.mr0().set_bit());

	// Set interrupt rising trigger for line 0
	exti.rtsr.modify(|_, w| w.tr0().set_bit());

	// Move control over to the global mutex
	cortex_m::interrupt::free(move |cs| {
		*INT.borrow(cs).borrow_mut() = Some(exti);
	});

    let mut nvic = cp.NVIC;
    nvic.enable(Interrupt::EXTI0);
	unsafe { nvic.set_priority(Interrupt::EXTI0, 1) };
	cortex_m::peripheral::NVIC::unpend(Interrupt::EXTI0);

    loop {
        cortex_m_semihosting::hprintln!(".").unwrap();
        delay.delay_ms(1_000_u16);
    }
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
