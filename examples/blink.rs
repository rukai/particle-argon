#![no_std]
#![no_main]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

use defmt::panic;
use defmt_rtt as _; // global logger
use panic_probe as _;
pub use defmt::*;

use core::sync::atomic::{AtomicUsize, Ordering};

defmt::timestamp! {"{=u64}", {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    // NOTE(no-CAS) `timestamps` runs with interrupts disabled
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n as u64
}}

use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy_nrf::Peripherals;

use particle_argon::{ModeButton, Led};

#[embassy::main]
async fn main(_spawner: Spawner) {
    let p = Peripherals::take().unwrap();
    let button = ModeButton::new(p.P0_11);
    let mut led = Led::new(p.P1_12);

    info!("started");

    loop {
        Timer::after(Duration::from_millis(300)).await;
        if button.is_held_down() {
            led.on();
            info!("led on");
        }
        else {
            led.off();
            info!("led off");
        }
    }
}
