//! Delays

use bsp::hal::{
    ccm::{Handle, IPGFrequency},
    gpt::GPT,
    pit::{channel::_3, PIT},
};
use embedded_hal::{
    blocking::delay::{DelayMs, DelayUs},
    prelude::_embedded_hal_timer_CountDown,
};
use teensy4_bsp as bsp;

/// System timer (SysTick) as a delay provider
pub struct Delay {
    timer: PIT<_3>,
}

impl Delay {
    /// Configures the system timer (SysTick) as a delay provider
    pub fn new(handle: &mut Handle, gpt: &mut GPT, timer: PIT<_3>) -> Self {
        gpt.set_mode(bsp::hal::gpt::Mode::FreeRunning);
        gpt.set_enable(true);

        Delay { timer }
    }

    /// Releases the system timer (SysTick) resource
    #[allow(dead_code)]
    pub fn free(self) -> PIT<_3> {
        self.timer
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
        self.timer.start(core::time::Duration::from_micros(us));
        nb::block!(self.timer.wait()).unwrap();
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
