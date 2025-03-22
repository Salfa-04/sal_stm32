//!
//! # Tasks
//!

use hal::{bind_interrupts, peripherals};
use {crate::hal, embassy_executor::task};

// pub use gps_task::gps_task;
pub use led_task::led_task;
pub use pwm_task::pwm_set_duty_cycle;
pub use pwm_task::pwm_task;
// pub use uart_task::uart_task;

bind_interrupts! {
    struct IntRqst {
        USART1 => hal::usart::InterruptHandler<peripherals::USART1>;
        USART3 => hal::usart::InterruptHandler<peripherals::USART3>;
    }
}

// mod gps_task;
mod led_task;
mod pwm_task;
// mod uart_task;
