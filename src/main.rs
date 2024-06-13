#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::Timer;
use hal::gpio::{AnyPin, Level, Output, Pin};
use hal::println;
use {ch32_hal as hal, panic_halt as _};


#[embassy_executor::task(pool_size = 2)]
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
    hal::debug::SDIPrint::enable();
    let p = hal::init(hal::Config::default());
    hal::embassy::init();

    spawner.spawn(blink(p.PC4.degrade(), 110)).unwrap();

    loop {
        Timer::after_millis(1000).await;
        println!("tick");
    }
}
