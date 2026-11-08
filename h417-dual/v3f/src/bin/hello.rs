//! Minimal dual-core example — V3F (boot) half.
//!
//! Reset only starts this core. Bring the chip up, then wake hart 1 at the V5F
//! flash origin. The V5F half (`v5f` crate, same bin name) blinks LED1 (PF0).
//!
//! ```text
//! cargo xtask flash --example hello
//! ```

#![no_std]
#![no_main]

use ch32_hal as hal;
use panic_halt as _;
use qingke::pfic;

/// Must match `v5f/memory.x` FLASH ORIGIN (1KB-aligned).
const V5F_ENTRY: u32 = 0x0001_0000;

#[ch32_hal::entry]
fn main() -> ! {
    // Only the boot core runs the bring-up: RCC, AFIO/GPIO and EXTI are shared.
    let _p = hal::init(hal::Config::default());

    unsafe { pfic::wake_other_core(V5F_ENTRY) };

    loop {
        hal::delay::Delay.delay_ms(1000);
    }
}
