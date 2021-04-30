#![no_main]
#![no_std]

use nrf52840_hal as hal;
use rtt_target::{rprintln, rtt_init_print};
use particle_argon::{ModeButton, Led};

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let port1 = hal::gpio::p1::Parts::new(p.P1);
    let button = ModeButton::new(port0.p0_11);
    let mut led  = Led::new(port1.p1_12);

    rprintln!("Blinky button demo starting");
    loop {
        if button.is_held_down() {
            led.on();
        } else {
            led.off();
        }
    }
}
