//! Delays

use bsp::hal::{
    ccm::{perclk::Multiplexer, Handle, IPGFrequency},
    gpt::{Unclocked, GPT},
    pit::{
        channel::{_0, _1, _2, _3},
        UnclockedPIT, PIT,
    },
};
use embedded_hal::{
    blocking::delay::{DelayMs, DelayUs},
    prelude::_embedded_hal_timer_CountDown,
};
use teensy4_bsp as bsp;

/// System timer (SysTick) as a delay provider
pub struct Delay {
    timer0: PIT<_0>,
    timer1: PIT<_1>,
    timer2: PIT<_2>,
    timer3: PIT<_3>,
}

impl Delay {
    /// Configures the system timer (SysTick) as a delay provider
    pub fn new(
        handle: &mut Handle,
        perclk: Multiplexer,
        gpt: Unclocked,
        pit: UnclockedPIT,
        ipg_hz: IPGFrequency,
    ) -> (GPT, Self) {
        let mut cfg = perclk.configure(
            handle,
            bsp::hal::ccm::perclk::PODF::DIVIDE_3,
            bsp::hal::ccm::perclk::CLKSEL::IPG(ipg_hz),
        );
        let mut gpt2 = gpt.clock(&mut cfg);
        gpt2.set_mode(bsp::hal::gpt::Mode::FreeRunning);
        gpt2.set_enable(true);

        let (timer0, timer1, timer2, timer3) = pit.clock(&mut cfg);

        (
            gpt2,
            Delay {
                timer0,
                timer1,
                timer2,
                timer3,
            },
        )
    }

    /// Releases the system timer (SysTick) resource
    #[allow(dead_code)]
    pub fn free(self) -> (PIT<_0>, PIT<_1>, PIT<_2>, PIT<_3>) {
        (self.timer0, self.timer1, self.timer2, self.timer3)
    }
}

impl DelayMs<u64> for Delay {
    fn delay_ms(&mut self, ms: u64) {
        self.delay_us(ms * 1_000);
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_ms(ms as u64);
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(ms as u64);
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(ms as u64);
    }
}

impl DelayUs<u64> for Delay {
    fn delay_us(&mut self, us: u64) {
        self.timer3.start(core::time::Duration::from_micros(us));
        nb::block!(self.timer3.wait()).unwrap();
    }
}
impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        self.delay_us(us as u64)
    }
}

impl DelayUs<u16> for Delay {
    fn delay_us(&mut self, us: u16) {
        self.delay_us(us as u64)
    }
}

impl DelayUs<u8> for Delay {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(us as u64)
    }
}
