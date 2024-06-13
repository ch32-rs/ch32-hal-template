#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

{% if embassy -%}
use embassy_executor::Spawner;
use embassy_time::Timer;
use hal::gpio::{AnyPin, Level, Output, Pin};
use hal::println;
use {ch32_hal as hal, panic_halt as _};

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
    hal::debug::SDIPrint::enable();
    let p = hal::init(hal::Config::default());
    hal::embassy::init();

    // Adjust the LED GPIO according to your board
    spawner.spawn(blink(p.PD6.degrade(), 100)).unwrap();

    loop {
        Timer::after_millis(1000).await;
        println!("tick");
    }
}
{%- else -%}
use hal::delay::Delay;
use hal::gpio::{Level, Output};
use {ch32_hal as hal, panic_halt as _};

#[qingke_rt::entry]
fn main() -> ! {
    hal::debug::SDIPrint::enable();
    let p = hal::init(hal::Config::default());

    let mut delay = Delay;

    // Adjust the LED GPIO according to your board
    let mut led = Output::new(p.PD6, Level::Low, Default::default());

    loop {
        led.toggle();

        delay.delay_ms(1000);
        hal::println!("toggle!");
    }
}
{%- endif %}