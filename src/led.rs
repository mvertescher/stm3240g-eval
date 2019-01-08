//! On-board user LEDs

use stm32f4xx_hal::prelude::_embedded_hal_digital_OutputPin;

use hal::gpio::gpioc::PC7;
use hal::gpio::gpiog::{PG6, PG8};
use hal::gpio::gpioi::PI9;
use hal::gpio::{Output, PushPull};

/// Far right green LED
pub type LD1 = PG6<Output<PushPull>>;

/// Center right orange LED
pub type LD2 = PG8<Output<PushPull>>;

/// Center left red LED
pub type LD3 = PI9<Output<PushPull>>;

/// Far left blue LED
pub type LD4 = PC7<Output<PushPull>>;

/// One of the on-board user LEDs
pub struct Led {
    pin: _embedded_hal_digital_OutputPin,
}

impl Led {
    /// Turns the LED off
    pub fn off(&mut self) {
        self.pin.set_low()
    }

    /// Turns the LED on
    pub fn on(&mut self) {
        self.pin.set_high()
    }
}
