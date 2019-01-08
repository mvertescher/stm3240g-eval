//! Board Support Crate for the STM3240G-EVAL

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

pub extern crate stm32f4xx_hal as hal;

pub mod led;
