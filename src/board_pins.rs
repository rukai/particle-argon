//! Type aliases from board pin names to nrf52840 pins.
//!
//! You are probably better off not using these.
//! It's easier to just think about the pin naming solely from the nrf52840 perspective.
//! But it's provided in case you do want to use it or just as documentation.

use nrf52840_hal::gpio::p0::*;
use nrf52840_hal::gpio::p1::*;

// Right hand side

/// Data pin for I2C or digital GPIO.
pub type SDA<T> = P0_26<T>;
/// Clock pin for I2C or digital GPIO.
pub type SCL<T> = P0_27<T>;
/// Digital GPIO. PWM-able.
pub type D2<T> = P1_01<T>;
/// Digital GPIO. PWM-able.
pub type D3<T> = P1_02<T>;
/// Digital GPIO. PWM-able.
pub type D4<T> = P1_08<T>;
/// Digital GPIO. PWM-able.
pub type D5<T> = P1_10<T>;
/// Digital GPIO. PWM-able.
pub type D6<T> = P1_11<T>;
/// Digital GPIO. PWM-able.
pub type D7<T> = P1_12<T>;
/// Digital GPIO. PWM-able.
pub type D8<T> = P1_03<T>;

// Left hand side

/// TX pin for UART or digital GPIO.
pub type TX<T> = P0_06<T>;
/// RX pin for UART or digital GPIO.
pub type RX<T> = P0_08<T>;
/// MI pin for SPI or digital GPIO.
pub type MI<T> = P1_14<T>;
/// MO pin for SPI or digital GPIO.
pub type MO<T> = P1_13<T>;
/// SCK pin for SPI or digital GPIO.
pub type SCK<T> = P1_15<T>;
/// Analog input pin or digital GPIO. PWM-able.
pub type A5<T> = P0_31<T>;
/// Analog input pin or digital GPIO. PWM-able.
pub type A4<T> = P0_30<T>;
/// Analog input pin or digital GPIO. PWM-able.
pub type A3<T> = P0_29<T>;
/// Analog input pin or digital GPIO. PWM-able.
pub type A2<T> = P0_28<T>;
/// Analog input pin or digital GPIO. PWM-able.
pub type A1<T> = P0_04<T>;
/// Analog input pin or digital GPIO. PWM-able.
pub type A0<T> = P0_03<T>;
/// Active-low MODE button pin.
pub type MD<T> = P0_11<T>;
/// Active-low system reset button pin. Internally pulled-up.
pub type RST<T> = P0_18<T>;
