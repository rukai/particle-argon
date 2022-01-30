#![no_std]

pub mod board_pins;

use embassy::time::{Duration, Timer};
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::peripherals::{P0_11, P0_16, P0_24, P1_12};
use embedded_hal::digital::v2::InputPin;

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
            pin: Output::new(pin, Level::Low, OutputDrive::Standard),
        }
    }

    /// Turn the LED on.
    pub fn on(&mut self) {
        self.pin.set_high();
    }

    /// Turn the LED off.
    pub fn off(&mut self) {
        self.pin.set_low();
    }
}

pub struct EspDriver {
    // TODO: store parts for EspAT
    _chip_pu: Output<'static, P0_24>,
    _boot_firmware: Output<'static, P0_16>,
}

impl EspDriver {
    /// Initialize the EspDriver
    pub async fn new(chip_pu: P0_24, boot_firmware: P0_16) -> Self {
        let _boot_firmware = Output::new(boot_firmware, Level::High, OutputDrive::Standard);

        let mut _chip_pu = Output::new(chip_pu, Level::Low, OutputDrive::Standard0Disconnect1);
        Timer::after(Duration::from_millis(1)).await;

        _chip_pu.set_high();

        Timer::after(Duration::from_millis(1000)).await; // TODO: how long?
        EspDriver {
            _chip_pu,
            _boot_firmware,
        }
    }
}

//macro_rules! turn_on_uart_and_get_at_driver {
//    // TODO: grab parts from wifi and return an EspAt
//}
