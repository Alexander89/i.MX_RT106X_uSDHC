#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// The uSDHC module can select the following modes for data transfer
///
/// ### NOTE:
///
/// This block can support all the above listed speed mode and
/// maximum clock frequency. However, these may be specific to
/// the device. See the corresponding chip-specific information or
/// the device data sheet for accurate details.
pub enum CardMode {
    Unknown,
    /// SD 1-bit
    Sd1,
    /// SD 4-bit
    Sd4,
    /// MMC 1-bit
    Mmc1,
    /// MMC 4-bit
    Mmc4,
    /// MMC 8-bit
    Mmc8,
    /// Identification mode (up to 400 kHz)
    Identification,
    /// MMC full-speed mode (up to 26 MHz)
    MmcFullSpeed,
    /// MMC high-speed mode (up to 52 MHz)
    MmcHighSpeed,
    /// MMC HS200 mode(up to 200 MHz)
    MmcHs200,
    /// MMC DDR mode (52 MHz both edges)
    MmcDdr,
    /// SD/SDIO full-speed mode (up to 25 MHz)
    SdSdioFullSpeed,
    /// SD/SDIO high-speed mode (up to 50 MHz)
    SdSdioHighSpeed,
    /// SD/SDIO UHS-I mode (up to 208 MHz in SDR mode,
    /// up to 50 MHz in the DDR mode)
    SdSdioUhsI,
}

pub const SD_MAX_INIT_RATE_HZ: u32 = 400_000;
pub const SD_MAX_MMC_FULL_SPEED_RATE_HZ: u32 = 26_000_000;
pub const SD_MAX_MMC_HIGH_SPEED_RATE_HZ: u32 = 52_000_000;
pub const SD_MAX_MMC_HS200_RATE_HZ: u32 = 200_000_000;
pub const SD_MAX_MMC_DDR_RATE_HZ: u32 = 52_000_000;
