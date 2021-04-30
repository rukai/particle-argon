#![no_std]

pub mod board_pins;

use nrf52840_hal::gpio::p0::P0_11;
use nrf52840_hal::gpio::p1::P1_12;
use nrf52840_hal::gpio::{Input, Output, PullUp, PushPull, Disconnected, Level};
use nrf52840_hal::prelude::*;

/// Simple abstraction around the MODE button connected to the MD/P0.11 pin.
pub struct ModeButton {
    pin: P0_11<Input<PullUp>>,
}

impl ModeButton {
    /// Initialize the button.
    pub fn new(pin: P0_11<Disconnected>) -> Self {
        ModeButton {
            pin: pin.into_pullup_input(),
        }
    }

    /// Returns true iff button is physically held down.
    pub fn is_held_down(&self) -> bool {
        self.pin.is_low().unwrap()
    }
}

/// Simple abstraction around the user LED at the top right of the board button connected to the D7/P1.12 pin.
pub struct Led {
    pin: P1_12<Output<PushPull>>,
}

impl Led {
    /// Initialize the LED, LED remains off.
    pub fn new(pin: P1_12<Disconnected>) -> Self {
        Led {
            pin: pin.into_push_pull_output(Level::Low),
        }
    }

    /// Turn the LED on.
    pub fn on(&mut self) {
        self.pin.set_high().unwrap();
    }

    /// Turn the LED off.
    pub fn off(&mut self) {
        self.pin.set_low().unwrap();
    }
}
