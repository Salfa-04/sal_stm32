#![no_std]
#![no_main]

use utils::prelude::*;

mod tasks;
mod utils;

#[embassy_executor::main]
async fn entry(s: embassy_executor::Spawner) {
    let (p,) = utils::sys_init();

    {
        // !TASK: led_task
        let p = (p.PC7,);
        s.must_spawn(tasks::led_task(p));
    }

    // {
    //     // !TASK: pwm_task
    //     let p = (p.TIM3, p.PA6, p.PA7, p.PB0, p.PB1);
    //     s.must_spawn(tasks::pwm_task(p));
    // }

    {
        // !TASK: rc_task
        let p = (p.USART6, p.PA12, p.DMA2_CH1);
        s.must_spawn(tasks::rc_task(p));
    }

    // {
    //     // !TASK: dbg_task
    //     let p = (p.USART1, p.PA10, p.PA9, p.DMA2_CH7, p.DMA2_CH2);
    //     s.must_spawn(tasks::dbg_task(p));
    // }
}
