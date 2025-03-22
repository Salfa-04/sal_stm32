//!
//! # UART Task
//!

use crate::{hal, init_ticker};

use embassy_executor::SendSpawner;
use hal::{mode::Async, peripherals, usart};
use peripherals::{DMA1_CH4, DMA1_CH5, PA9, PA10, USART1};
use static_cell::StaticCell;
use usart::{Config, Uart, UartRx, UartTx};

// use embassy_sync::zerocopy_channel::Channel;

#[super::task]
async fn uart_rx_task(rx: &'static mut UartRx<'static, Async>) -> ! {
    let mut buffer = [0u8; 128];

    loop {
        match rx.read_until_idle(&mut buffer).await {
            Ok(x) => defmt::info!("USART Read: {:?}", buffer[..x]),
            Err(e) => defmt::error!("USART Read Error: {:?}", e),
        };
    }
}

#[super::task]
async fn uart_tx_task(tx: &'static mut UartTx<'static, Async>) -> ! {
    let mut t = init_ticker!(100);

    let buffer = b"sending..\r\n";

    loop {
        tx.write(buffer).await.unwrap();
        // tx.send_break();

        t.next().await;
    }
}

#[super::task]
pub async fn uart_task(spaw: SendSpawner, p: (USART1, PA10, PA9, DMA1_CH4, DMA1_CH5)) {
    let mut config = Config::default();
    config.baudrate = 115200;

    let (tx, rx) = {
        static UART: StaticCell<Uart<Async>> = StaticCell::new();
        UART.init(
            Uart::new(p.0, p.1, p.2, super::IntRqst, p.3, p.4, config)
                .inspect_err(|e| defmt::error!("UART Init Error: {:?}", e))
                .unwrap(),
        )
        .split_ref()
    };

    spaw.must_spawn(uart_rx_task(rx));
    spaw.must_spawn(uart_tx_task(tx));
}
