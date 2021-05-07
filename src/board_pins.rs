//! Type aliases from board pin names to nrf52840 pins.
//!
//! You are probably better off not using these.
//! It's easier to just think about the pin naming solely from the nrf52840 perspective.
//! But it's provided in case you do want to use it or just as documentation.

use embassy_nrf::peripherals::*;

// Right hand side

/// Data pin for I2C or digital GPIO.
pub type SDA = P0_26;
/// Clock pin for I2C or digital GPIO.
pub type SCL = P0_27;
/// Digital GPIO. PWM-able.
pub type D2 = P1_01;
/// Digital GPIO. PWM-able.
pub type D3 = P1_02;
/// Digital GPIO. PWM-able.
pub type D4 = P1_08;
/// Digital GPIO. PWM-able.
pub type D5 = P1_10;
/// Digital GPIO. PWM-able.
pub type D6 = P1_11;
/// Digital GPIO. PWM-able.
pub type D7 = P1_12;
/// Digital GPIO. PWM-able.
pub type D8 = P1_03;

// Left hand side

/// TX pin for UART or digital GPIO.
pub type TX = P0_06;
/// RX pin for UART or digital GPIO.
pub type RX = P0_08;
/// MI pin for SPI or digital GPIO.
pub type MI = P1_14;
/// MO pin for SPI or digital GPIO.
pub type MO = P1_13;
/// SCK pin for SPI or digital GPIO.
pub type SCK = P1_15;
/// Analog input pin or digital GPIO. PWM-able.
pub type A5 = P0_31;
/// Analog input pin or digital GPIO. PWM-able.
pub type A4 = P0_30;
/// Analog input pin or digital GPIO. PWM-able.
pub type A3 = P0_29;
/// Analog input pin or digital GPIO. PWM-able.
pub type A2 = P0_28;
/// Analog input pin or digital GPIO. PWM-able.
pub type A1 = P0_04;
/// Analog input pin or digital GPIO. PWM-able.
pub type A0 = P0_03;
/// Active-low MODE button pin.
pub type MD = P0_11;
/// Active-low system reset button pin. Internally pulled-up.
pub type RST = P0_18;
