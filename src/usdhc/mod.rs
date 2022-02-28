mod block_transfer;
mod buffer;
pub mod commands;
mod constants;
mod crc;
mod mode_switch;
mod sd_card;

use core::marker::PhantomData;

pub use constants::*;
use hal::{
    gpio,
    iomuxc::{self, consts::U1},
    ral::{
        modify_reg,
        usdhc::{CMD_XFR_TYP::RSPTYP, INT_STATUS_EN::DMAESEN::RW::DMAESEN_1},
    },
};
use teensy4_bsp::{
    self as bsp,
    hal::{
        self,
        ral::{self},
    },
    interrupt,
    pins::imxrt_iomuxc::consts::Unsigned,
    pins::imxrt_iomuxc::usdhc,
};

use crate::usdhc::commands::SdAppOpCond;

use self::commands::{BcCommand, BcrCommand};

/*/
fn send_command(cmd_index, cmd_arg, other requirements)
{
    WORD wCmd;  // 32-bit integer to make up the data to write into Transfer Type register, it is recommended to implement in a bit-field manner
    wCmd = (<cmd_index> & 0x3f) << 24; // set the first 8 bits as '00'+<cmd_index> set CMDTYP, DPSEL, CICEN, CCCEN, RSTTYP, DTDSEL accorind to the command index;
    if (internal DMA is used){
        wCmd |= 0x1;
    }
    if (multi-block transfer) {
        set MSBSEL bit;
        if (finite block number) {
            set BCEN bit;
            if (auto12 command is to use) {
                set AC12EN bit
            };
        }
    }
    write_reg(CMDARG, <cmd_arg>); // configure the command argument
    write_reg(XFERTYP, wCmd); // set Transfer Type register as wCmd value to issue the command
}
wait_for_response(cmd_index)
{
    while (CC bit in IRQ Status register is not set) {} // wait until Command Complete bit is set
    // read IRQ Status register and check if any error bits about Command are set
    if (any error bits are set) {
        report error;
    }
    write 1 to clear CC bit and all Command Error bits;
}

*/

pub struct Uninitialized<M> {
    usdhc: ral::usdhc::Instance,
    _module: PhantomData<M>,
}

impl<M> Uninitialized<M> {
    pub fn new(usdhc: ral::usdhc::Instance) -> Self {
        // log::debug!("0 {:b}", ral::read_reg!(ral::usdhc, usdhc, PRES_STATE));
        // log::debug!("1 {:b}", ral::read_reg!(ral::usdhc, usdhc, SYS_CTRL));
        // log::debug!("2 {:b}", ral::read_reg!(ral::usdhc, usdhc, INT_STATUS));
        // log::debug!("3 {:b}", ral::read_reg!(ral::usdhc, usdhc, INT_STATUS_EN));
        // log::debug!("4 {:b}", ral::read_reg!(ral::usdhc, usdhc, INT_SIGNAL_EN));
        // log::debug!("5 {:b}", ral::read_reg!(ral::usdhc, usdhc, MIX_CTRL));

        Self {
            usdhc,
            _module: PhantomData::default(),
        }
    }

    /// Builds an usdhc peripheral from the CMD, CLK, D0, D1, D2, D3 pins.
    /// The return is a configured usdhc interface.
    pub fn clock(self, ccm: &mut hal::ccm::Handle) -> Builder<U1> {
        let (ccm, _) = ccm.raw();

        // disable before configuration
        ral::modify_reg!(ral::ccm, ccm, CCGR6, CG1: 0, CG2: 0);
        // no divider here !?

        ral::modify_reg!(ral::ccm, ccm, CSCDR1, USDHC1_PODF: 0b001);

        // run with 396 52MHz
        ral::modify_reg!(ral::ccm, ccm, CSCMR1, USDHC1_CLK_SEL: 0b01);

        // enable after configuration
        ral::modify_reg!(ral::ccm, ccm, CCGR6, CG1: 0b11, CG2: 0b11);

        Builder::new(self.usdhc)
    }
}

pub struct Builder<M> {
    _module: PhantomData<M>,
    usdhc_reg: ral::usdhc::Instance,
}

impl<M> Builder<M>
where
    M: Unsigned,
{
    pub fn new(usdhc_reg: ral::usdhc::Instance) -> Self {
        //ral::write_reg!(ral::usdhc, usdhc_reg, PRES_STATE, 0u32);
        log::debug!("0 {:b}", ral::read_reg!(ral::usdhc, usdhc_reg, PRES_STATE));

        Self {
            _module: PhantomData,
            usdhc_reg,
        }
    }

    pub fn build<CMD, CLK, D0, D1, D2, D3>(
        self,
        cmd: CMD,
        clk: CLK,
        d0: D0,
        d1: D1,
        d2: D2,
        d3: D3,
    ) -> USdhc<M, CMD, CLK, D0, D1, D2, D3>
    where
        CMD: usdhc::Pin<Module = M, Signal = usdhc::Cmd>,
        CLK: usdhc::Pin<Module = M, Signal = usdhc::Clk>,
        D0: usdhc::Pin<Module = M, Signal = usdhc::Data0>,
        D1: usdhc::Pin<Module = M, Signal = usdhc::Data1>,
        D2: usdhc::Pin<Module = M, Signal = usdhc::Data2>,
        D3: usdhc::Pin<Module = M, Signal = usdhc::Data3>,
    {
        ral::modify_reg!(ral::usdhc, self.usdhc_reg, MIX_CTRL, DTDSEL: 0);
        let pins = USdhcPins::new(cmd, clk, d0, d1, d2, d3);

        USdhc::new(self.usdhc_reg, pins)
    }
}

struct USdhcPins<M, CMD, CLK, D0, D1, D2, D3>
where
    M: Unsigned,
    CMD: usdhc::Pin<Module = M, Signal = usdhc::Cmd>,
    CLK: usdhc::Pin<Module = M, Signal = usdhc::Clk>,
    D0: usdhc::Pin<Module = M, Signal = usdhc::Data0>,
    D1: usdhc::Pin<Module = M, Signal = usdhc::Data1>,
    D2: usdhc::Pin<Module = M, Signal = usdhc::Data2>,
    D3: usdhc::Pin<Module = M, Signal = usdhc::Data3>,
{
    cmd: CMD,
    clk: CLK,
    d0: D0,
    d1: D1,
    d2: D2,
    d3: D3,
}

impl<M, CMD, CLK, D0, D1, D2, D3> USdhcPins<M, CMD, CLK, D0, D1, D2, D3>
where
    M: Unsigned,
    CMD: usdhc::Pin<Module = M, Signal = usdhc::Cmd>,
    CLK: usdhc::Pin<Module = M, Signal = usdhc::Clk>,
    D0: usdhc::Pin<Module = M, Signal = usdhc::Data0>,
    D1: usdhc::Pin<Module = M, Signal = usdhc::Data1>,
    D2: usdhc::Pin<Module = M, Signal = usdhc::Data2>,
    D3: usdhc::Pin<Module = M, Signal = usdhc::Data3>,
{
    fn new(cmd: CMD, clk: CLK, d0: D0, d1: D1, d2: D2, d3: D3) -> Self {
        Self {
            cmd,
            clk,
            d0,
            d1,
            d2,
            d3,
        }
    }

    pub fn enable(&mut self) {
        usdhc::prepare(&mut self.cmd);
        usdhc::prepare(&mut self.clk);
        usdhc::prepare(&mut self.d0);
        usdhc::prepare(&mut self.d1);
        usdhc::prepare(&mut self.d2);
        usdhc::prepare(&mut self.d3);
    }

    pub fn disable(&mut self) {
        iomuxc::alternate(&mut self.cmd, 5);
        iomuxc::alternate(&mut self.clk, 5);
        iomuxc::alternate(&mut self.d0, 5);
        iomuxc::alternate(&mut self.d1, 5);
        iomuxc::alternate(&mut self.d2, 5);
        iomuxc::alternate(&mut self.d3, 5);
    }
}

pub struct USdhc<M, CMD, CLK, D0, D1, D2, D3>
where
    M: Unsigned,
    CMD: usdhc::Pin<Module = M, Signal = usdhc::Cmd>,
    CLK: usdhc::Pin<Module = M, Signal = usdhc::Clk>,
    D0: usdhc::Pin<Module = M, Signal = usdhc::Data0>,
    D1: usdhc::Pin<Module = M, Signal = usdhc::Data1>,
    D2: usdhc::Pin<Module = M, Signal = usdhc::Data2>,
    D3: usdhc::Pin<Module = M, Signal = usdhc::Data3>,
{
    usdhc: ral::usdhc::Instance,
    mode: CardMode,
    pins: USdhcPins<M, CMD, CLK, D0, D1, D2, D3>,
    sd_clk_khz: u32,
}

impl<M, CMD, CLK, D0, D1, D2, D3> USdhc<M, CMD, CLK, D0, D1, D2, D3>
where
    M: Unsigned,
    CMD: usdhc::Pin<Module = M, Signal = usdhc::Cmd>,
    CLK: usdhc::Pin<Module = M, Signal = usdhc::Clk>,
    D0: usdhc::Pin<Module = M, Signal = usdhc::Data0>,
    D1: usdhc::Pin<Module = M, Signal = usdhc::Data1>,
    D2: usdhc::Pin<Module = M, Signal = usdhc::Data2>,
    D3: usdhc::Pin<Module = M, Signal = usdhc::Data3>,
{
    fn new(usdhc: ral::usdhc::Instance, mut pins: USdhcPins<M, CMD, CLK, D0, D1, D2, D3>) -> Self {
        pins.enable();
        Self {
            usdhc,
            pins,
            mode: CardMode::Unknown,
            sd_clk_khz: 0,
        }
    }

    fn base_clock(ccm: &mut hal::ccm::Handle) -> u32 {
        let (ccm, ccm_analog) = ccm.raw();
        let divider = ral::read_reg!(ral::ccm, ccm, CSCDR1, USDHC1_PODF) + 1;
        let fq = ral::read_reg!(ral::ccm_analog, ccm_analog, PFD_528) & 0x3F;
        let res = (528_000_000_u32 * 3) / (fq / 6) / divider;
        log::debug!("fq {}, divider {}, res {}", fq, divider, res);
        return res;
    }
    fn set_sd_clk(&mut self, max_sd_clk: u32, ccm: &mut hal::ccm::Handle) {
        let dvs_limit: u32 = 0x10;
        let sd_clk_fs_limit: u32 = 0x100;
        let base: u32 = Self::base_clock(ccm);

        // while ((base/(sdclkfs*DVS_LIMIT) > maxSdclk) && (sdclkfs < SDCLKFS_LIMIT)) {
        //  sdclkfs <<= 1;
        // }
        // while ((base/(sdclkfs*dvs) > maxSdclk) && (dvs < DVS_LIMIT)) {
        //  dvs++;
        // }
        // m_sdClkKhz = base/(1000*sdclkfs*dvs);
        // sdclkfs >>= 1;
        // dvs--;

        let mut sd_clk_fs: u32 = 1;
        while (base / (sd_clk_fs * dvs_limit) > max_sd_clk) && (sd_clk_fs < sd_clk_fs_limit) {
            sd_clk_fs <<= 1;
        }

        let mut dvs: u32 = 1;
        while (base / (sd_clk_fs * dvs) > max_sd_clk) && (dvs < dvs_limit) {
            dvs += 1;
        }

        self.sd_clk_khz = base / (1000 * sd_clk_fs * dvs);
        log::debug!("clk khz {}", self.sd_clk_khz);

        sd_clk_fs = sd_clk_fs >> 1;
        dvs -= 1;

        // #if defined(__MK64FX512__) || defined(__MK66FX1M0__)
        //   // Disable SDHC clock.
        //   SDHC_SYSCTL &= ~SDHC_SYSCTL_SDCLKEN;
        // #endif  // defined(__MK64FX512__) || defined(__MK66FX1M0__)
        //   // Change dividers.
        //   uint32_t sysctl = SDHC_SYSCTL & ~(SDHC_SYSCTL_DTOCV_MASK
        //     | SDHC_SYSCTL_DVS_MASK | SDHC_SYSCTL_SDCLKFS_MASK);
        //  // SDHC_SYSCTL = sysctl | SDHC_SYSCTL_DTOCV(0x0E) | SDHC_SYSCTL_DVS(dvs)
        //         | SDHC_SYSCTL_SDCLKFS(sdclkfs);
        //  // // Wait until the SDHC clock is stable.
        // while (!(SDHC_PRSSTAT & SDHC_PRSSTAT_SDSTB)) {
        // }

        while ral::read_reg!(ral::usdhc, self.usdhc, PRES_STATE, SDSTB == 0) {}

        ral::modify_reg!(
            ral::usdhc,
            self.usdhc,
            VEND_SPEC,
            FRC_SDCLK_ON: 0
        );

        // Change dividers.
        ral::modify_reg!(
            ral::usdhc,
            self.usdhc,
            SYS_CTRL,
            DTOCV: DTOCV_14,
            DVS: dvs,
            SDCLKFS: sd_clk_fs
        );

        // Wait until the SDHC clock is stable.
        while ral::read_reg!(ral::usdhc, self.usdhc, PRES_STATE, SDSTB == 0) {}

        ral::modify_reg!(
            ral::usdhc,
            self.usdhc,
            VEND_SPEC,
            FRC_SDCLK_ON: 1
        );
        // #if defined(__MK64FX512__) || defined(__MK66FX1M0__)
        //   // Enable the SDHC clock.
        //   SDHC_SYSCTL |= SDHC_SYSCTL_SDCLKEN;
        // #endif  // defined(__MK64FX512__) || defined(__MK66FX1M0__)
    }

    fn set_init_mode(&mut self, ccm: &mut hal::ccm::Handle) {
        if self.mode != CardMode::Identification {
            self.pins.disable();

            self.reset_and_wait();

            self.set_sd_clk(constants::SD_MAX_INIT_RATE_HZ, ccm);

            self.pins.enable();

            self.mode = CardMode::Identification;
        }
    }

    pub fn init(&mut self, ccm: &mut hal::ccm::Handle) {
        self.set_init_mode(ccm);

        #[cortex_m_rt::interrupt]
        fn USDHC1() {
            unsafe {
                // WHY ?
                ral::write_reg!(ral::usdhc, ral::usdhc::USDHC1, INT_SIGNAL_EN, 0);

                let int = ral::read_reg!(ral::usdhc, ral::usdhc::USDHC1, INT_STATUS);
                log::debug!("int {:b}", int);
                ral::write_reg!(ral::usdhc, ral::usdhc::USDHC1, INT_STATUS, int);
                ral::modify_reg!(ral::usdhc, ral::usdhc::USDHC1, MIX_CTRL, AC23EN: 0, DMAEN: DMAEN_0);
            }

            /*
                1_1000_0000_0000_0000

            SDHC_IRQSIGEN = 0;
            m_irqstat = SDHC_IRQSTAT;
            SDHC_IRQSTAT = m_irqstat;
            #if defined(__IMXRT1062__)
            SDHC_MIX_CTRL &= ~(SDHC_MIX_CTRL_AC23EN | SDHC_MIX_CTRL_DMAEN);
            #endif
            */
        }
        self.enable_interrupts();

        /*
        // Disable GPIO clock.
            (false);

        SDHC_MIX_CTRL |= 0x80000000;

        // Reset SDHC. Use default Water Mark Level of 16.
        SDHC_SYSCTL |= SDHC_SYSCTL_RSTA | SDHC_SYSCTL_SDCLKFS(0x80);

        while (SDHC_SYSCTL & SDHC_SYSCTL_RSTA) {
        }

        // Set initial SCK rate.
        setSdclk(SD_MAX_INIT_RATE_KHZ);

        enableGPIO(true);

        SDHC_IRQSTATEN = SDHC_IRQSTATEN_MASK;

        attachInterruptVector(IRQ_SDHC, sdIrs);
        NVIC_SET_PRIORITY(IRQ_SDHC, 6*16);
        NVIC_ENABLE_IRQ(IRQ_SDHC);
        */
    }

    pub fn enable_interrupts(&mut self) {
        cortex_m::peripheral::NVIC::mask(bsp::interrupt::USDHC1);

        // Enable desired IRQSTAT bits.
        ral::write_reg!(
            ral::usdhc,
            self.usdhc,
            INT_SIGNAL_EN,
            TCIEN: TCIEN_1,
            DMAEIEN: DMAEIEN_1,
            AC12EIEN: AC12EIEN_1,
            DEBEIEN: DEBEIEN_1,
            DCEIEN: DCEIEN_1,
            DTOEIEN: DTOEIEN_1,
            CIEIEN: CIEIEN_1,
            CEBEIEN: CEBEIEN_1,
            CCEIEN: CCEIEN_1,
            CTOEIEN: CTOEIEN_1,
            DINTIEN: DINTIEN_1,
            CCIEN: CCIEN_1,
            CINSIEN: CINSIEN_1,
            CRMIEN: CRMIEN_1
        );

        ral::write_reg!(ral::usdhc, self.usdhc, INT_STATUS_EN, 0xffff_ffff);

        cortex_m::interrupt::free(|_cs| {
            // Safety: invoked in a critical section that also prepares the ISR
            // shared memory. ISR memory is ready by the time the ISR runs.
            unsafe { cortex_m::peripheral::NVIC::unmask(bsp::interrupt::USDHC1) };
        });
    }
    pub fn wait_for_card(&mut self) {
        while self.is_card_busy() {
            // log::debug!("---")
        }
    }
    pub fn wait_for_command_complete(&mut self) {
        while !self.is_command_comlete() {}
    }

    pub fn send_command(&mut self, cmd: impl commands::SdCommand) -> bool {
        if cmd.req_app_cmd() {
            log::debug!("send req app cmd");
            // recursive call with AppCmd (CMD55)
            self.send_command(commands::AppCmd::new(0));
        }
        log::debug!("send cmd: {}", cmd.cmd_id());

        self.wait_for_card();

        ral::write_reg!(ral::usdhc, self.usdhc, CMD_ARG, cmd.mk_args());
        ral::write_reg!(ral::usdhc, self.usdhc, CMD_XFR_TYP, cmd.mk_xfer());

        self.wait_for_command_complete();
        self.wait_for_card();

        let (cc, cie, cebe, cce, ctoe) =
            ral::read_reg!(ral::usdhc, self.usdhc, INT_STATUS, CC, CIE, CEBE, CCE, CTOE);
        ral::modify_reg!(
            ral::usdhc,
            self.usdhc,
            INT_STATUS,
            CC: cc,
            CIE: cie,
            CEBE: cebe,
            CCE: cce,
            CTOE: ctoe
        );
        return (cc == 1) && (cie + cebe + cce + ctoe == 0);
    }

    pub fn read_response_u32(&self) -> u32 {
        ral::read_reg!(ral::usdhc, self.usdhc, CMD_RSP0)
    }

    pub fn init_card(&mut self) {
        ral::modify_reg!(ral::usdhc, self.usdhc, SYS_CTRL, INITA: 1);

        while ral::read_reg!(ral::usdhc, self.usdhc, SYS_CTRL, INITA == 1) {}

        /*
                 DBG_IRQSTAT();
          if (waitTimeout(isBusyCommandInhibit)) {
            return false;  // Caller will set errorCode.
          }
          SDHC_CMDARG = arg;
        #if defined(__IMXRT1062__)
          // Set MIX_CTRL if data transfer.
          if (xfertyp & SDHC_XFERTYP_DPSEL) {
            SDHC_MIX_CTRL &= ~SDHC_MIX_CTRL_MASK;
            SDHC_MIX_CTRL |= xfertyp & SDHC_MIX_CTRL_MASK;
          }
          xfertyp &= ~SDHC_MIX_CTRL_MASK;
        #endif  // defined(__IMXRT1062__)
          SDHC_XFERTYP = xfertyp;
          if (waitTimeout(isBusyCommandComplete)) {
            return false;  // Caller will set errorCode.
          }
          m_irqstat = SDHC_IRQSTAT;
          SDHC_IRQSTAT = m_irqstat;

          return (m_irqstat & SDHC_IRQSTAT_CC) &&
                 !(m_irqstat & SDHC_IRQSTAT_CMD_ERROR);
                  */
    }
}

impl<M, CMD, CLK, D0, D1, D2, D3> USdhc<M, CMD, CLK, D0, D1, D2, D3>
where
    M: Unsigned,
    CMD: usdhc::Pin<Module = M, Signal = usdhc::Cmd>,
    CLK: usdhc::Pin<Module = M, Signal = usdhc::Clk>,
    D0: usdhc::Pin<Module = M, Signal = usdhc::Data0>,
    D1: usdhc::Pin<Module = M, Signal = usdhc::Data1>,
    D2: usdhc::Pin<Module = M, Signal = usdhc::Data2>,
    D3: usdhc::Pin<Module = M, Signal = usdhc::Data3>,
{
    fn reset_and_wait(&mut self) {
        ral::modify_reg!(ral::usdhc, self.usdhc, SYS_CTRL, RSTA: 1, SDCLKFS: 0x80);
        while ral::read_reg!(ral::usdhc, self.usdhc, SYS_CTRL, RSTA == 1) {}
    }

    pub fn is_card_inserted(&mut self) -> bool {
        let dlsl = ral::read_reg!(ral::usdhc, self.usdhc, PRES_STATE, DLSL);
        (dlsl & 0b001000) != 0
    }

    pub fn get_state(&mut self) -> u32 {
        ral::read_reg!(ral::usdhc, self.usdhc, PRES_STATE)
    }
    pub fn get_clock_state(&mut self) -> u32 {
        unsafe { ral::read_reg!(ral::usdhc, hal::ral::ccm::CCM, CCGR6) }
    }
    pub fn is_card_busy(&mut self) -> bool {
        ral::read_reg!(ral::usdhc, self.usdhc, PRES_STATE, CIHB == CIHB_1)
    }
    fn is_command_comlete(&mut self) -> bool {
        let (cc, cie, cebe, cce, ctoe) =
            ral::read_reg!(ral::usdhc, self.usdhc, INT_STATUS, CC, CIE, CEBE, CCE, CTOE);

        (cc + cie + cebe + cce + ctoe) == 0
    }

    // 1111_0111___1000_1000
    // 1000_0000___1000_1000

    /// set the block size and count for the next command
    ///
    /// - count max = 65535
    /// - size max = 4096
    pub fn set_block_size_and_count(&mut self, count: u32, size: u32) {
        ral::write_reg!(ral::usdhc, self.usdhc, BLK_ATT, BLKCNT: count.min(65535), BLKSIZE: size.min(4096));
    }
}
