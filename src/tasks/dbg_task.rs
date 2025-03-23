//!
//! # UART Task
//!

use crate::hal::{mode::Async, peripherals, usart};
use defmt::{error, info};
use embassy_sync::{blocking_mutex::raw, mutex};
use peripherals::{DMA1_CH4, DMA1_CH5, PA9, PA10, USART1};
use usart::{Config, Uart, UartTx};
use {mutex::Mutex as M, raw::ThreadModeRawMutex as RM};

pub static DBG_TX: M<RM, Option<UartTx<Async>>> = M::new(None);

/// Print to the UART Tx
///
/// ## Please ensure that `UART Tx` is set before using this macro
///
#[macro_export]
macro_rules! uprintln {
    ($($arg:tt)*) => {
        use embedded_io::Write as  _;
        let mut tx = $crate::tasks::DBG_TX.lock().await;
        if let Some(tx) = tx.as_mut() {
            if let Err(e) = writeln!(tx, $($arg)*) {
                ::defmt::error!("Debug Tx Error: {:?}", e);
            }
        } else {
            panic!("Debug TX not Initialized!");
        }
    };
}

#[super::task]
pub async fn dbg_task(p: (USART1, PA10, PA9, DMA1_CH4, DMA1_CH5)) {
    let mut config = Config::default();
    config.baudrate = 115200;

    let (tx, mut rx) = Uart::new(p.0, p.1, p.2, super::IntRqst, p.3, p.4, config)
        .inspect_err(|e| error!("UART Init Error: {:?}", e))
        .unwrap()
        .split();

    if let Some(_) = DBG_TX.lock().await.replace(tx) {
        panic!("Debug Tx Already Initialized!");
    };

    info!("USART1 Initialized!");

    let mut buffer = [0u8; 128];

    loop {
        match rx.read_until_idle(&mut buffer).await {
            Ok(x) => {
                // super::pwm_task::pwm_set_dbg(&buffer[..x]).await;

                // do something with the data
                info!("USART Read: {:?}", buffer[..x]);
            }
            Err(e) => error!("USART Read Error: {:?}", e),
        };
    }
}
