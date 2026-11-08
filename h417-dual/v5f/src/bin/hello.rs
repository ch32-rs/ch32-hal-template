//! Minimal dual-core example — V5F half.
//!
//! Woken by the V3F `hello` image. The boot core already configured the clock
//! tree and GPIO clocks, so this hart must not call `ch32_hal::init` again.
//! It toggles LED1 (PF0) through the PAC.
//!
//! ```text
//! cargo xtask flash --example hello
//! ```

#![no_std]
#![no_main]

use ch32_hal::pac;
use panic_halt as _;

/// Port F, pin 0 (`LED1` on the EVT board).
const PORT_F: usize = 5;
const PIN: usize = 0;

#[qingke_rt::entry]
fn main() -> ! {
    let gpiof = pac::GPIO(PORT_F);

    // The PAC names each CNF encoding after its input-mode meaning: CNF=00 is
    // "analog in / push-pull out". A plain LED wants that.
    gpiof.cfglr().modify(|w| {
        w.set_mode(PIN, pac::gpio::vals::Mode::OUTPUT_50MHZ);
        w.set_cnf(PIN, pac::gpio::vals::Cnf::ANALOG_IN__PUSH_PULL_OUT);
    });

    loop {
        gpiof.bshr().write(|w| w.set_bs(PIN, true));
        spin();
        gpiof.bshr().write(|w| w.set_br(PIN, true));
        spin();
    }
}

/// Crude delay: `hal::delay::Delay` is calibrated by the boot core's
/// `hal::init()`, so hart 1 spins instead.
fn spin() {
    for _ in 0..5_000_000 {
        unsafe { core::arch::asm!("nop") };
    }
}
