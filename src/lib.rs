#![no_std]

pub mod board_pins;

use embassy_nrf::peripherals::{P0_11, P1_12};
use embassy_nrf::gpio::{Level, Output, Input, Pull, OutputDrive};
use embedded_hal::digital::v2::{OutputPin, InputPin};

/// Simple abstraction around the MODE button connected to the MD/P0.11 pin.
pub struct ModeButton {
    pin: Input<'static, P0_11>,
}

impl ModeButton {
    /// Initialize the button.
    pub fn new(pin: P0_11) -> Self {
        ModeButton {
            pin: Input::new(pin, Pull::Up),
        }
    }

    /// Returns true iff button is physically held down.
    pub fn is_held_down(&self) -> bool {
        self.pin.is_low().unwrap()
    }
}

/// Simple abstraction around the user LED at the top right of the board connected to the D7/P1.12 pin.
pub struct Led {
    pin: Output<'static, P1_12>,
}

impl Led {
    /// Initialize the LED, LED remains off.
    pub fn new(pin: P1_12) -> Self {
        Led {
            pin: Output::new(pin, Level::Low, OutputDrive::Standard), //TODO: PushPull
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
