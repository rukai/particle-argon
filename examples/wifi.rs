#![no_std]
#![no_main]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

use esp_at_driver::{EspAt, WifiMode};

use defmt::panic;
use embassy::time::{Duration, Timer};
use embassy::executor::Spawner;
use embassy::util::Steal;
use embassy_nrf::gpio::NoPin;
use embassy_nrf::{interrupt, uarte, Peripherals};
use embassy_nrf::buffered_uarte::BufferedUarte;
use futures::pin_mut;

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

#[embassy::main]
async fn main(_spawner: Spawner) {
    let p = unsafe { Peripherals::steal() };

    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD115200;

    let irq = interrupt::take!(UARTE0_UART0);

    let mut tx_buffer = [0u8; 4096];
    let mut rx_buffer = [0u8; 4096];
    let uart = unsafe {
        BufferedUarte::new(
            p.UARTE0,
            p.TIMER0,
            p.PPI_CH0,
            p.PPI_CH1,
            irq,
            p.P0_08,
            p.P0_06,
            NoPin,
            NoPin,
            config,
            &mut rx_buffer,
            &mut tx_buffer,
        )
    };

    info!("uart initialized!");

    pin_mut!(uart);
    let mut esp_at = EspAt::new(uart);

    loop {
        esp_at.set_wifi_mode(WifiMode::Station).await.unwrap();
        info!("set wifi mode...");
        Timer::after(Duration::from_millis(10000)).await;
    }
}
