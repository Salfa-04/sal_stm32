#![no_std]
#![no_main]

use utils::prelude::*;

mod libs;
mod tasks;
mod utils;

#[embassy_executor::main]
async fn entry(s: embassy_executor::Spawner) {
    let (p,) = utils::sys_init();

    {
        // !TASK: led_task
        let p = (p.PC13,);
        s.must_spawn(tasks::led_task(p));
    }

    {
        // !TASK: pwm_task
        let p = (p.TIM3, p.PA6, p.PA7, p.PB0, p.PB1);
        s.must_spawn(tasks::pwm_task(p));
    }

    // {
    //     use pid::Pid;

    //     let mut a: Pid<f32> = Pid::new(0f32, 90f32);
    //     a.kp = 0.5;
    //     a.ki = 0.1;
    //     a.kd = 0.1;

    //     a.p_limit;
    // }

    // {
    //     // !TASK: dbg_task
    //     let p = (p.USART1, p.PA10, p.PA9, p.DMA1_CH4, p.DMA1_CH5);
    //     s.must_spawn(tasks::dbg_task(p));
    // }
}
