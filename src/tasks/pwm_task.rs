//!
//! # LED Task
//!

use crate::{hal, init_ticker};
use hal::{gpio, peripherals, time, timer};

use defmt::{debug, error};
use gpio::OutputType;
use peripherals::{PA6, PA7, TIM3};
use time::hz;
use timer::low_level::CountingMode::EdgeAlignedUp;
use timer::simple_pwm::{PwmPin, SimplePwm};

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::once_lock::OnceLock;
use embassy_sync::signal::Signal;

static MAX_DUTY_CYCLE: OnceLock<u16> = OnceLock::new();
static DUTY_CYCLE_X: Signal<ThreadModeRawMutex, f32> = Signal::new();
static DUTY_CYCLE_Y: Signal<ThreadModeRawMutex, f32> = Signal::new();

/// Duty Cycle Update Prop
const DUTY_PROP: f32 = 1f32;

pub async fn pwm_set_duty_cycle(x: i8, y: i8) {
    assert!(x >= -90 && x <= 90);
    assert!(y >= -90 && y <= 90);

    let max = *MAX_DUTY_CYCLE.get().await;

    // duty_cycle = x / 90° + 1.5
    //          x = -90° to 90°
    DUTY_CYCLE_X.signal((x as f32 / 90f32 + 1.5f32) * max as f32 / 20f32);
    DUTY_CYCLE_Y.signal((y as f32 / 90f32 + 1.5f32) * max as f32 / 20f32);
}

#[super::task]
pub async fn pwm_task(p: (TIM3, PA6, PA7)) -> ! {
    let mut t = init_ticker!(10);

    let pin_x = PwmPin::new_ch1(p.1, OutputType::PushPull);
    let pin_y = PwmPin::new_ch2(p.2, OutputType::PushPull);

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
            debug!("Max Duty Cycle X: {}", max_duty_cycle_x);
            debug!("Max Duty Cycle Y: {}", max_duty_cycle_y);
            assert!(max_duty_cycle_x == max_duty_cycle_y);
            max_duty_cycle_x
        })
        .unwrap();

    let mut duty_cycle_x = 0f32;
    let mut duty_cycle_y = 0f32;

    let mut duty_cycle_inner_x = 0f32;
    let mut duty_cycle_inner_y = 0f32;

    loop {
        if DUTY_CYCLE_X.signaled() {
            if let Some(x) = DUTY_CYCLE_X.try_take() {
                duty_cycle_x = x;
            } else {
                error!("Duty Cycle X is not taken");
            }
        }

        if DUTY_CYCLE_Y.signaled() {
            if let Some(x) = DUTY_CYCLE_Y.try_take() {
                duty_cycle_y = x;
            } else {
                error!("Duty Cycle Y is not taken");
            }
        }

        if duty_cycle_x > duty_cycle_inner_x {
            duty_cycle_inner_x += DUTY_PROP;
        } else if duty_cycle_x < duty_cycle_inner_x {
            duty_cycle_inner_x -= DUTY_PROP;
        }

        if duty_cycle_y > duty_cycle_inner_y {
            duty_cycle_inner_y += DUTY_PROP;
        } else if duty_cycle_y < duty_cycle_inner_y {
            duty_cycle_inner_y -= DUTY_PROP;
        }

        ch_x.set_duty_cycle(duty_cycle_inner_x as u16);
        ch_y.set_duty_cycle(duty_cycle_inner_y as u16);

        t.next().await;
    }
}
