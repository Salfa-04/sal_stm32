//!
//! # Tasks
//!

use hal::{bind_interrupts, peripherals};
use {crate::hal, embassy_executor::task};

mod dbg_task;
pub use dbg_task::dbg_task;

bind_interrupts! {
    struct IntRqst {
        USART1 => hal::usart::InterruptHandler<peripherals::USART1>;
        USART3 => hal::usart::InterruptHandler<peripherals::USART3>;
    }
}

mod led_task;
pub use led_task::led_task;

mod pwm_task;
pub use pwm_task::pwm_set_duty_cycle;
pub use pwm_task::pwm_task;
