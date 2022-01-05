#![no_std]
#![no_main]

mod delay;
mod usb_io;
mod usdhc;

use bsp::interrupt;
use delay::Delay;

use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use teensy4_bsp as bsp;
use teensy4_panic as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    run();
}

fn run() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let _ = bsp::t41::from_pads(peripherals.iomuxc);
    usb_io::init();

    let (_, ipg_hz) = peripherals.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut peripherals.ccm.handle,
        &mut peripherals.dcdc,
    );
    let (_, mut delay) = Delay::new(
        &mut peripherals.ccm.handle,
        peripherals.ccm.perclk,
        peripherals.gpt2,
        peripherals.pit,
        ipg_hz,
    );
    delay.delay_ms(3000u64);
    log::debug!("init tempmon");

    loop {
        usdhc::test();
    }
}
