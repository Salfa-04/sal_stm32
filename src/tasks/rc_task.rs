//!
//! # UART Task
//!

use crate::hal::{peripherals, usart};
use defmt::{Format, error, info};

static RC_CTL: Mutex<ThreadModeRawMutex, RcCtl> = Mutex::new(RcCtl::new());

#[derive(Default, Debug, Format)]
pub struct RcCtl {
    ch_l_horiz: i16,
    ch_l_verti: i16,
    ch_r_horiz: i16,
    ch_r_verti: i16,
    ch_roller: i16,
    sw_left: SwValue,
    sw_right: SwValue,

    mouse_x: i16,
    mouse_y: i16,
    mouse_z: i16,
    mouse_left: u8,
    mouse_right: u8,
    key_value: u16,
}

#[derive(Default, Debug, Format)]
pub enum SwValue {
    #[default]
    SwMid,
    SwUp,
    SwDown,
}

impl RcCtl {
    const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    fn update(&mut self, data: &[u8]) {
        let data = if data.len() == 18 {
            &data[..18]
        } else if data.len() == 18 * 2 {
            &data[18..18 * 2]
        } else {
            return;
        };

        assert_eq!(data.len(), 18);

        self.ch_r_horiz = ((data[0] as u16) | (data[1] as u16) << 8) as i16 & 0x7FF;
        self.ch_r_verti = ((data[1] as u16) >> 3 | (data[2] as u16) << 5) as i16 & 0x7FF;
        self.ch_l_horiz =
            ((data[2] as u16) >> 6 | (data[3] as u16) << 2 | (data[4] as u16) << 10) as i16 & 0x7FF;
        self.ch_l_verti = ((data[4] as u16) >> 1 | (data[5] as u16) << 7) as i16 & 0x7FF;
        self.ch_roller = ((data[16] as u16) | (data[17] as u16) << 8) as i16 & 0x7FF;

        const CH_VALUE_OFFSET: i16 = 1024;
        self.ch_r_horiz -= CH_VALUE_OFFSET;
        self.ch_r_verti -= CH_VALUE_OFFSET;
        self.ch_l_horiz -= CH_VALUE_OFFSET;
        self.ch_l_verti -= CH_VALUE_OFFSET;
        self.ch_roller -= CH_VALUE_OFFSET;

        assert!(self.ch_l_horiz >= -660 && self.ch_l_horiz <= 660);
        assert!(self.ch_l_verti >= -660 && self.ch_l_verti <= 660);
        assert!(self.ch_r_horiz >= -660 && self.ch_r_horiz <= 660);
        assert!(self.ch_r_verti >= -660 && self.ch_r_verti <= 660);
        assert!(self.ch_roller >= -660 && self.ch_roller <= 660);

        self.sw_left = match data[5] >> 6 & 0x3 {
            1 => SwValue::SwUp,
            3 => SwValue::SwMid,
            2 => SwValue::SwDown,
            _ => unreachable!(),
        };
        self.sw_right = match data[5] >> 4 & 0x3 {
            1 => SwValue::SwUp,
            3 => SwValue::SwMid,
            2 => SwValue::SwDown,
            _ => unreachable!(),
        };

        self.key_value = ((data[14] as u16) | (data[15] as u16) << 8) as u16;
        self.mouse_x = ((data[6] as u16) | (data[7] as u16) << 8) as i16;
        self.mouse_y = ((data[8] as u16) | (data[9] as u16) << 8) as i16;
        self.mouse_z = ((data[10] as u16) | (data[11] as u16) << 8) as i16;

        self.mouse_left = data[12];
        self.mouse_right = data[13];

        assert!(self.mouse_left >> 1 == 0);
        assert!(self.mouse_right >> 1 == 0);
    }
}

use embassy_sync::{self as sync, mutex::Mutex};
use peripherals::{DMA2_CH1, PA12, USART6};
use sync::blocking_mutex::raw::ThreadModeRawMutex;
use usart::{Config, Parity, UartRx};

#[super::task]
pub async fn rc_task(p: (USART6, PA12, DMA2_CH1)) {
    let mut config = Config::default();
    config.baudrate = 100000;
    config.parity = Parity::ParityEven;

    let mut u = UartRx::new(p.0, super::IntRqst, p.1, p.2, config)
        .inspect_err(|e| error!("UART Init Error: {:?}", e))
        .unwrap();

    info!("Remote Ctroller Initialized!");

    let mut buffer = [0u8; 48];

    loop {
        match u.read_until_idle(&mut buffer).await {
            Ok(x) => {
                RC_CTL.lock().await.update(&buffer[..x]);
                // info!("RC Update: {:?}", rc_ctl);
            }

            Err(e) => error!("RC Read Error: {:?}", e),
        };
    }
}
