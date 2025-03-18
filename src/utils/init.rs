//!
//! # Init
//!

use crate::hal::{Config, init, rcc, time::mhz};

pub fn sys_init() -> (embassy_stm32::Peripherals,) {
    let peripherals = {
        let mut config = Config::default();
        let rcc = &mut config.rcc;

        rcc.hsi = false;
        rcc.hse = Some(rcc::Hse {
            freq: mhz(8),
            mode: rcc::HseMode::Oscillator,
        });

        rcc.sys = rcc::Sysclk::PLL1_P;
        rcc.pll = Some(rcc::Pll {
            src: rcc::PllSource::HSE,
            prediv: rcc::PllPreDiv::DIV1,
            mul: rcc::PllMul::MUL9,
        });

        rcc.ahb_pre = rcc::AHBPrescaler::DIV1;
        rcc.apb1_pre = rcc::APBPrescaler::DIV2;
        rcc.apb2_pre = rcc::APBPrescaler::DIV1;
        rcc.adc_pre = rcc::ADCPrescaler::DIV6;
        rcc.mux = rcc::mux::ClockMux::default();
        rcc.ls = rcc::LsConfig::default_lse();

        init(config)
    };

    ::defmt::info!("Init: Peripherals!");

    (peripherals,)
}
