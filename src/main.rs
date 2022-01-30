#![no_std]
#![no_main]

mod delay;
mod usb_io;
mod usdhc;

use bsp::{
    hal::{self, gpio::GPIO},
    Led,
};
use delay::Delay;

use embedded_hal::{digital::v2::OutputPin, prelude::_embedded_hal_blocking_delay_DelayMs};
use hal::iomuxc::consts::U1;
use teensy4_bsp as bsp;
use teensy4_panic as _;
use usdhc::commands::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    run();
}

fn run() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let pins = bsp::t41::from_pads(peripherals.iomuxc);
    usb_io::init();

    let (_, ipg_hz) = peripherals.ccm.pll1.set_arm_clock(
        bsp::hal::ccm::PLL1::ARM_HZ,
        &mut peripherals.ccm.handle,
        &mut peripherals.dcdc,
    );
    peripherals.ccm.pll2.set(
        &mut peripherals.ccm.handle,
        [
            Some(bsp::hal::ccm::pll2::MHZ_352),
            Some(bsp::hal::ccm::pll2::MHZ_594),
            Some(bsp::hal::ccm::pll2::MHZ_396),
            Some(bsp::hal::ccm::pll2::MHZ_297),
        ],
    );
    let mut cfg = peripherals.ccm.perclk.configure(
        &mut peripherals.ccm.handle,
        bsp::hal::ccm::perclk::PODF::DIVIDE_1,
        bsp::hal::ccm::perclk::CLKSEL::IPG(ipg_hz),
    );

    let mut gpt2 = peripherals.gpt2.clock(&mut cfg);
    let pit = peripherals.pit.clock(&mut cfg);
    let mut delay = Delay::new(&mut peripherals.ccm.handle, &mut gpt2, pit.3);

    delay.delay_ms(2500u64);

    let uninit_sd = hal::ral::usdhc::USDHC1::take()
        .map(|inst| usdhc::Uninitialized::<U1>::new(inst))
        .expect("where are all thous flowers gone");

    let mut sd = uninit_sd
        .clock(&mut peripherals.ccm.handle)
        .build(pins.p45, pins.p44, pins.p43, pins.p42, pins.p47, pins.p46);

    let state = sd.get_state();
    sd.wait_for_card();

    log::debug!("init state {:b}", state);

    sd.init(&mut peripherals.ccm.handle);

    let mut led = Led::from(GPIO::new(pins.p13).output());

    let state = sd.get_state();
    log::debug!("state {:b}", state);

    // sd.enable_interrupts();

    log::warn!("GoIdleState {}", sd.set_command(GoIdleState::new()));

    let state = sd.get_state();
    log::debug!("state {:b}", state);

    let mut m_version2 = false;

    for _ in 0..3 {
        if sd.set_command(SendExtCsd::new()) {
            let resp = sd.read_response_u32();
            if resp != 0x1AA {
                log::error!("wrong resp {}", resp);
            } else {
                m_version2 = true;
            }
            break;
        } else {
            log::warn!("err")
        }
    }
    log::warn!("version 2 {}", m_version2);

    let orc = if m_version2 {
        0x40300000_u32
    } else {
        0x00300000_u32
    };

    let res = loop {
        sd.set_command(SdAppOpCond::new(orc));

        let res = sd.read_response_u32();
        if (res & 0x80000000) != 0 {
            break res;
        }
    };

    log::error!("SdAppOpCond {}", res);

    // sd.set_command(SendExtCsd::new());

    delay.delay_ms(5000u64);

    loop {
        let state = sd.get_state();
        log::debug!("state {:b}", state);

        let state = sd.is_card_inserted();
        led.set_state(state.into());

        delay.delay_ms(500u64);
    }
}
