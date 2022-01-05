//! # uSDHC module data buffer
//!
//! The uSDHC module uses one configurable data buffer to transfer data between the
//! system bus (IP bus or advanced high-performance bus (AHB) bus) and the SD card in an
//! optimized manner, maximizing throughput between the two clock domains (IP peripheral
//! clock and the master clock).
//!
//! The buffer is used as a temporary storage for transferring data between the host system
//! and the card. The watermark levels for read and write are both configurable and can
//! range between 1 (`BUFFER_MIN_WATERMARK`) to 128 (`BUFFER_MAX_WATERMARK`) words. The burst
//! lengths for read and write are also configurable and can range between
//! 1 (`BUFFER_MIN_BURST_LENGTH`) to 31 (`BUFFER_MAX_BURST_LENGTH`) words.

/// The watermark levels (**RD_WML**) for read and write are both
/// configurable and can range between 1 `BUFFER_MIN_WATERMARK`
/// to 128 words
pub const BUFFER_MAX_WATERMARK: u8 = 128;
pub const BUFFER_MIN_WATERMARK: u8 = 1;

/// The burst lengths for read and write are also
/// configurable and can range between 1 `BUFFER_MIN_BURST_LENGTH`
/// to 31 words
pub const BUFFER_MAX_BURST_LENGTH: u8 = 31;
pub const BUFFER_MIN_BURST_LENGTH: u8 = 1;

/// Here are 2 transfer modes to access the data buffer:
/// - CPU polling mode
/// - Internal DMA mode (includes simple and advanced DMA accesses):
enum BufferReadTransferMode {
    /// For a host-read operation, when the number of words received in the buffer
    /// meets or exceeds the RD_WML watermark value, by polling the BRR bit, the
    /// host driver can read the Buffer Data Port register to fetch the amount of words
    /// set in the RD_WML register from the buffer. The write operation is similar. For
    /// more information on the process of writing operation, see Write operation
    /// sequence.
    CpuPolling,
    /// The internal DMA access, either by simple or advanced DMA, is over the AHB bus
    InternalDma,
}

// read operation
/*
For a read operation, when there are more words in the buffer than the amount set in the
RD_WML register, the internal DMA starts fetching data over the AHB bus. Except for
INCR4 and INCR8, the burst type is always the INCR mode and the burst length depends
on the shortest of the following factors:

 - Burst length configured in the burst length field of the Watermark Level register
 - Watermark level boundary
 - Block size boundary
 - Data boundary configured in the current descriptor (if the ADMA is active)
 - 1 KB address boundary defined in the AHB protocol
 */

/// There are 2 ways to write data into the buffer when the user transfers data to the card:
/// - Processor core polling
/// - Internal DMA
enum BufferWriteTransferMode {
    /// Processor core polling through the BWR bit in the Interrupt Status register (interrupt or polling)
    CpuPolling,
    /// The internal DMA access, either by simple or advanced DMA, is over the AHB bus
    InternalDma,
}

/*
When the internal DMA is not used, the DMAEN bit in the Transfer Type register is not
set when the command is sent, uSDHC asserts a DMA request when the amount of buffer
space exceeds the value set in the WR_WML register and is ready for receiving new data.
At the same time, uSDHC sets the BWR bit. The buffer write ready interrupt is generated
if it is enabled by software.

When internal DMA is used, uSDHC does not inform the system before all the required
number of bytes are transferred (if no error is encountered). When an error occurs during
the data transfer, uSDHC aborts the data transfer and abandons the current block. The
host driver should read the contents of the DMA System Address register to obtain the
starting address of the abandoned data block. If the current data transfer is in multi-block
mode, uSDHC does not automatically send CMD12, even though the AC12EN bit in the
Transfer Type register is set. The host driver sends CMD12 in this scenario and restarts
the write operation from that address. It is recommended that a software reset for Data be
applied before the transfer is restarted.

The uSDHC module does not start data transmission until the buffer has been filled with
the number of words set in the WR_WML register. If the buffer is empty and the host
system does not write data in time, uSDHC stops the CLK to avoid the data buffer
underrun situation.
 */
