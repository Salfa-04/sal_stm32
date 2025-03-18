//!
//! ..
//!

#![no_std]
#![no_main]
#![deny(missing_docs)]

use utils::prelude::*;

mod utils;

#[embassy_executor::main]
async fn entry(s: embassy_executor::Spawner) {
    let (p,) = utils::sys_init();

    s.must_spawn(led_task(p.PC13));
}

#[embassy_executor::task]
async fn led_task(led_pin: hal::peripherals::PC13) {
    use hal::gpio::{Level, OutputOpenDrain as OP, Speed};

    let mut led = OP::new(led_pin, Level::High, Speed::Low);

    loop {
        T::after_millis(300).await;
        led.toggle();
    }
}
