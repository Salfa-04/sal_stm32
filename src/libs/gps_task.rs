//!
//! # GPS Task
//!
//! ## NMEA Parameters Used
//!
//! - GGA
//! - GLL
//! - GSA
//! - GSV
//!
//! - RMC
//! - VTG
//! - ZDA
//!
//! - TXT
//! - DHV
//!

// use crate::hal;
use crate::hal::{peripherals, usart};

use defmt::{Debug2Format, debug, error};
use peripherals::{DMA1_CH3, PB11, USART3};
use usart::{Config as C, UartRx as P};

#[super::task]
pub async fn gps_task(p: (USART3, PB11, DMA1_CH3)) -> ! {
    // let mut led = OP::new(p.0, Level::High, Speed::Low);
    let mut config = C::default();
    config.baudrate = 9600;

    let mut rx = P::new(p.0, super::IntRqst, p.1, p.2, config)
        .inspect_err(|e| error!("USART Init Error: {:?}", e))
        .unwrap();

    let mut buffer = [0u8; 64];

    // nmea = { version = "0.7", features = [
    //     "defmt-03",
    //     "all-sentences",
    // ], default-features = false }

    // {
    //     // !TASK: gps_task
    //     let p = (p.USART3, p.PB11, p.DMA1_CH3);
    //     s.must_spawn(tasks::gps_task(p));
    // }

    loop {
        match rx.read_until_idle(&mut buffer).await {
            Ok(x) => {
                match nmea::parse_bytes(&buffer[..x]) {
                    Ok(x) => debug!("NMEA: {:?}", x),
                    Err(e) => error!("NMEA Parse Error: {:?}", Debug2Format(&e)),
                };
            }

            Err(e) => error!("UART Read Error: {:?}", e),
        };
    }
}
