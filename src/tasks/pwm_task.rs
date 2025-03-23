//!
//! # LED Task
//!

use crate::{hal, init_ticker};
use defmt::{error, trace};
use hal::{gpio, peripherals, time::hz, timer};

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::once_lock::OnceLock;
use embassy_sync::signal::Signal;
use gpio::OutputType;
use peripherals::{PA6, PA7, PB0, PB1, TIM3};
use timer::low_level::CountingMode::EdgeAlignedUp;
use timer::simple_pwm::{PwmPin, SimplePwm};

static MAX_DUTY_CYCLE: OnceLock<u16> = OnceLock::new();
static DUTY_CYCLE: Signal<ThreadModeRawMutex, (f32, f32)> = Signal::new();

// use embassy_sync::{blocking_mutex::raw, mutex};
// use {mutex::Mutex as M, raw::ThreadModeRawMutex as RM};
// static DUTY_STEP_I: M<RM, f32> = M::new(0f32);

/// Duty Cycle Update Prop
/// ! 需要调整
/// 1° ~~ 36.36f32
const DUTY_STEP: f32 = 800f32;

/// PWM Duty Cycle Set
/// x, y: from -90 to 90
pub async fn pwm_set_duty_cycle(x: i8, y: i8) {
    // assert!(x >= -90 && x <= 90);
    // assert!(y >= -90 && y <= 90);

    let max = *MAX_DUTY_CYCLE.get().await;

    // duty_cycle = x / 90° + 1.5
    //          x = -90° to 90°
    DUTY_CYCLE.signal((
        (x as f32 + 135f32) * max as f32 / 1800f32,
        (y as f32 + 135f32) * max as f32 / 1800f32,
    ));
}

// pub async fn pwm_set_dbg(data: &[u8]) {
//     for (i, d) in data.split(|&x| x == b',').enumerate() {
//         let s = core::str::from_utf8(d).unwrap().trim();

//         defmt::info!("Data: {}", s);

//         match i {
//             0 => {
//                 if let Ok(n) = s.parse::<f32>() {
//                     *DUTY_STEP_I.lock().await = n;
//                 } else {
//                     error!("Invalid Data: {}", s);
//                 }
//             }
//             1 => {
//                 let n = s.parse::<i8>().unwrap();
//                 pwm_set_duty_cycle(n, n).await;
//             }

//             _ => {
//                 error!("Invalid Data: {}", s);
//             }
//         }
//     }
// }

#[super::task]
pub async fn pwm_task(p: (TIM3, PA6, PA7, PB0, PB1)) -> ! {
    let mut t = init_ticker!(20);

    let pin_x = PwmPin::new_ch1(p.1, OutputType::PushPull);
    let pin_y = PwmPin::new_ch2(p.2, OutputType::PushPull);
    let _rev = PwmPin::new_ch3(p.3, OutputType::PushPull);
    let _rev = PwmPin::new_ch4(p.4, OutputType::PushPull);

    let pwm = SimplePwm::new(
        p.0,
        Some(pin_x),
        Some(pin_y),
        None,
        None,
        hz(50),
        EdgeAlignedUp,
    );

    let channels = pwm.split();
    let mut ch_x = channels.ch1;
    let mut ch_y = channels.ch2;

    ch_x.enable();
    ch_y.enable();

    t.next().await;

    MAX_DUTY_CYCLE
        .init({
            let max_duty_cycle_x = ch_x.max_duty_cycle();
            let max_duty_cycle_y = ch_y.max_duty_cycle();
            trace!(
                "Max Duty Cycle [x, y]: [{},\t{}]",
                max_duty_cycle_x, max_duty_cycle_y
            );
            assert!(max_duty_cycle_x == max_duty_cycle_y);
            max_duty_cycle_x
        })
        .unwrap();

    let mut duty_cycle_x = 0f32;
    let mut duty_cycle_y = 0f32;

    let mut duty_cycle_x_step = 0f32;
    let mut duty_cycle_y_step = 0f32;

    loop {
        if DUTY_CYCLE.signaled() {
            if let Some((x, y)) = DUTY_CYCLE.try_take() {
                duty_cycle_x = x;
                duty_cycle_y = y;
            } else {
                error!("Duty Cycle is not taken");
            }
        }

        // #[allow(non_snake_case)]
        // let DUTY_STEP = *DUTY_STEP_I.lock().await;

        if duty_cycle_x > duty_cycle_x_step {
            duty_cycle_x_step += DUTY_STEP;
            if duty_cycle_x <= duty_cycle_x_step {
                duty_cycle_x_step = duty_cycle_x;
            }
        } else if duty_cycle_x < duty_cycle_x_step {
            duty_cycle_x_step -= DUTY_STEP;
            if duty_cycle_x >= duty_cycle_x_step {
                duty_cycle_x_step = duty_cycle_x;
            }
        }

        if duty_cycle_y > duty_cycle_y_step {
            duty_cycle_y_step += DUTY_STEP;
            if duty_cycle_y <= duty_cycle_y_step {
                duty_cycle_y_step = duty_cycle_y;
            }
        } else if duty_cycle_y < duty_cycle_y_step {
            duty_cycle_y_step -= DUTY_STEP;
            if duty_cycle_y >= duty_cycle_y_step {
                duty_cycle_y_step = duty_cycle_y;
            }
        }

        ch_x.set_duty_cycle(duty_cycle_x_step as u16);
        ch_y.set_duty_cycle(duty_cycle_y_step as u16);

        // defmt::info!("ix={},", duty_cycle_x_step);

        t.next().await;
    }
}
