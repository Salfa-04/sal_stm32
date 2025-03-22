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

    // {
    //     // !TASK: gps_task
    //     let p = (p.USART3, p.PB11, p.DMA1_CH3);
    //     s.must_spawn(tasks::gps_task(p));
    // }

    // {
    //     // !TASK: uart_task
    //     let p = (p.USART1, p.PA10, p.PA9, p.DMA1_CH4, p.DMA1_CH5);
    //     s.must_spawn(tasks::uart_task(s.make_send(), p));
    // }

    {
        // !TASK: pwm_task
        let p = (p.TIM3, p.PA6, p.PA7);
        s.must_spawn(tasks::pwm_task(p));
    }

    {
        let mut t = init_ticker!(1000);

        let x = 0;
        let y = 0;

        loop {
            tasks::pwm_set_duty_cycle(x, y).await;

            t.next().await;
        }
    }
}
