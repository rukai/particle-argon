#![no_std]
#![no_main]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

use particle_argon::{EspDriver, Led};
//use esp_at_driver::{EspAt, WifiMode, ConnectionConfig};

use defmt::panic;
use embassy::time::{Duration, Timer};
use embassy::executor::Spawner;
use embassy::util::Steal;
use embassy_nrf::{interrupt, uarte, Peripherals};
use embassy_nrf::buffered_uarte::BufferedUarte;
use futures::pin_mut;
//use futures::poll;

use defmt_rtt as _; // global logger
use panic_probe as _;
//pub use defmt::*;
pub use defmt::info;

use core::sync::atomic::{AtomicUsize, Ordering};

use embassy::io::{AsyncWriteExt, AsyncBufReadExt};

defmt::timestamp! {"{=u64}", {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    // NOTE(no-CAS) `timestamps` runs with interrupts disabled
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n as u64
}}

use embassy_nrf::system;

fn get_clock_config() -> system::Config {
    let mut config = system::Config::default();
    config.hfclk_source = system::HfclkSource::ExternalXtal;
    config.lfclk_source = system::LfclkSource::ExternalXtal;
    config
}

#[embassy::main(config = "get_clock_config()")]
async fn main(_spawner: Spawner) {
    let mut p = unsafe { Peripherals::steal() };

    let mut led = Led::new(p.P1_12);
    led.off();

    let mut irq = interrupt::take!(UARTE0_UART0);

    let _esp_driver = EspDriver::new(p.P0_24, p.P0_16).await;

    Timer::after(Duration::from_millis(3000)).await;
    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD921600;

    loop {
        let mut config = uarte::Config::default();
        config.parity = uarte::Parity::EXCLUDED;
        config.baudrate = uarte::Baudrate::BAUD921600;

        Timer::after(Duration::from_millis(1000)).await;
        info!("chilling...");
        let mut tx_buffer = [0u8; 4096];
        let mut rx_buffer = [0u8; 4096];
        let uart = unsafe {
            BufferedUarte::new(
                &mut p.UARTE0,
                &mut p.TIMER0,
                &mut p.PPI_CH0,
                &mut p.PPI_CH1,
                &mut irq,
                &mut p.P1_04,
                &mut p.P1_05,
                &mut p.P1_06,
                &mut p.P1_07,
                config,
                &mut rx_buffer,
                &mut tx_buffer,
            )
        };
        info!("uart initialized!");
        pin_mut!(uart);
        Timer::after(Duration::from_millis(1000)).await;
        info!("write AT");
        uart.write_all(b"AT\r\n").await.unwrap();
        Timer::after(Duration::from_millis(1000)).await;
        let foo = uart.read_buf().await.unwrap();
        info!("uart response: {}", core::str::from_utf8(foo).unwrap());
        //if let Ready(foo) = poll!(uart.poll_fill_buf()) {
        //    info!("foo: {}", foo);
        //}
        //else {
        //    info!("uh oh");
        //}
    }
//    let mut esp_at = EspAt::new(uart);
//
//    let http = r#"GET /super_secret_url HTTP/1.1
//Host: developer.mozilla.org
//Accept-Language: fr
//"#;
//
//    Timer::after(Duration::from_millis(10000)).await;
//    info!("before read!");
//    esp_at.direct_read().await.unwrap(); // TODO: lets try running a newer probe-rs maybe????
//    info!("after read!");
//    esp_at.set_wifi_mode(WifiMode::Station).await.unwrap();
//    info!("set wifi mode...");
//
//    Timer::after(Duration::from_millis(10000)).await;
//    esp_at.direct_read().await.unwrap();
//    let config = ConnectionConfig {
//        ssid: Some("OPTUS_64_FD8C"),
//        password: Some("bikerflosh77117"),
//        ..
//        ConnectionConfig::default()
//    };
//    esp_at.connect_to_access_point(config).await.unwrap();
//    Timer::after(Duration::from_millis(1000)).await;
//    esp_at.direct_read().await.unwrap();
//    info!("connect to access point...");
//
//    Timer::after(Duration::from_millis(10000)).await;
//    esp_at.direct_read().await.unwrap();
//    esp_at.direct_write(r#"AT+CIPSTART="TCP","192.168.0.105",8000\r\n"#.as_bytes()).await.unwrap();
//    Timer::after(Duration::from_millis(100)).await;
//    esp_at.direct_read().await.unwrap();
//    info!("connect to tcp socket...");
//
//    loop {
//        Timer::after(Duration::from_millis(10000)).await;
//        esp_at.direct_read().await.unwrap();
//        let mut send_command = String::<128>::new();
//        write!(send_command, "AT+CIPSEND={}\r\n", http.len()).unwrap();
//        esp_at.direct_write(send_command.as_bytes()).await.unwrap();
//        Timer::after(Duration::from_millis(100)).await;
//        esp_at.direct_read().await.unwrap();
//        info!("start send");
//
//        // waiting for ">"
//        Timer::after(Duration::from_millis(1000)).await;
//        esp_at.direct_read().await.unwrap();
//        esp_at.direct_write(http.as_bytes()).await.unwrap();
//        Timer::after(Duration::from_millis(100)).await;
//        esp_at.direct_read().await.unwrap();
//        info!("complete send");
//    }
}
