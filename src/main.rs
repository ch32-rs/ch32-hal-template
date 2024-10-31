#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::Timer;
use ch32_hal::gpio::{AnyPin, Level, Output, Pin};
use ch32_hal::println;
use panic_halt as _;


#[embassy_executor::task]
async fn blink(pin: AnyPin, interval_ms: u64) {
    let mut led = Output::new(pin, Level::Low, Default::default());

    loop {
        led.set_high();
        Timer::after_millis(interval_ms).await;
        led.set_low();
        Timer::after_millis(interval_ms).await;
    }
}


#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(spawner: Spawner) -> ! {
    ch32_hal::debug::SDIPrint::enable();
    let p = ch32_hal::init(ch32_hal::Config::default());

    // Adjust the LED GPIO according to your board
    spawner.spawn(blink(p.PA0.degrade(), 1000)).unwrap();

    loop {
        Timer::after_millis(1000).await;
        println!("tick");
    }
}
