/// Broadcast commands (bc), no response
enum BcCommands {
    /// ## CMD0
    ///
    /// Resets all MMC and SD memory cards to idle state.
    ///
    /// ## Arguments:
    /// [31:0] stuff bits
    GoIdleState = 1,
    /// ## CMD4
    ///
    /// Programs the DSR of all cards.
    ///
    /// ## Arguments:
    /// [31:16] DSR
    /// [15:0] stuff bits
    SetDsr = 4,
    /// ## CMD5
    ///
    /// Asks all SDIO cards in idle state to
    /// send them operation conditions
    /// register contents in the response
    /// on the CMD line.
    ///
    /// ## Arguments:
    /// [31:0] OCR without busy
    IoSendOpCond = 5,
}

/// Broadcast commands with response (bcr), response from all cards simultaneously
enum BcrCommands {
    /// ## CMD1
    ///
    /// Asks all MMC and SD Memory cards in idle state to
    /// send their operation conditions register contents in
    /// the response on the CMD line.
    ///
    /// ## Arguments:
    /// [31:0] OCR without busy
    ///
    /// response type: R3
    SendOpCond = 1,
    /// ## CMD2
    ///
    /// Asks all cards to send their CID numbers on the CMD line.
    ///
    /// ## Arguments:
    /// [31:0] stuff bits
    ///
    /// response type: R2
    AllSendCid = 2,
    /// ## CMD40
    ///
    /// Sets the system into interrupt mode.
    ///
    /// ## Arguments:
    /// [31:0] stuff bits
    ///
    /// response type: R5
    IoSendOpCond = 40,
}

/// Addressed (point-to-point) commands (ac), no data transfer on the DATA
enum AcCommands {
    /// ## CMD3
    ///
    /// Assigns relative address to the card.
    ///
    /// ## Arguments:
    /// [31:16] RCA
    /// [15:0] stuff bits
    ///
    /// response type: R1 / R6 (SDIO)
    SetSendRelativeAddr = 3,
    /// ## CMD6
    ///
    /// # Note:
    ///   CMD6 differs completely between high-speed MMC cards and high-speed SD cards.
    ///   This Command SWITCH is for **high-speed MMC** cards.
    ///
    /// Switches the mode of operation of the selected card or modifies the
    /// EXT_CSD registers. Refer to "The MultiMediaCard System Specification
    /// Version 4.0 Final draft 2" for more details.
    ///
    /// ## Arguments:
    /// [31:26] Set to 0  
    /// [25:24] Access
    /// [23:16] Index
    /// [15:8] Value
    /// [7:3] Set to 0
    /// [2:0] Cmd Set
    ///
    /// response type: R1b
    Switch = 6,
    /// ## CMD7
    ///
    /// Toggles a card between the stand- by and transfer states or between
    /// the programming and disconnect states. In both cases, the card is selected
    /// by its own relative address and gets deselected by any other address.
    /// Address 0 deselects all.
    ///
    /// ## Arguments:
    /// [31:16] RCA
    /// [15:0] stuff bits
    ///
    /// response type: R1b
    SelectDeselectCard = 7,
    /// ## CMD9
    ///
    /// Addressed card sends its card-specific data (CSD) on the CMD line.
    ///
    /// ## Arguments:
    /// [31:16] RCA
    /// [15:0] stuff bits
    ///
    /// response type: R2
    SendCsd = 9,
    /// ## CMD10
    ///
    /// Addressed card sends its card-identification (CID) on the CMD line.
    ///
    /// ## Arguments:
    /// [31:16] RCA
    /// [15:0] stuff bits
    ///
    /// response type: R2
    SendCid = 10,
    /// ## CMD12
    ///
    /// Addressed card sends its card-identification (CID) on the CMD line.
    ///
    /// ## Arguments:
    /// [31:0] stuff bits
    ///
    /// response type: R1b
    StopTransmission = 12,
    /// ## CMD13
    ///
    /// Addressed card sends its status register.
    ///
    /// ## Arguments:
    /// [31:16] RCA
    /// [15:0] stuff bits
    ///
    /// response type: R1b
    SendStatus = 13,
    /// ## CMD15
    ///
    /// Addressed card sends its status register.
    ///
    /// ## Arguments:
    /// [31:16] RCA
    /// [15:0] stuff bits
    GoInactiveState = 15,
    /// ## CMD16
    ///
    /// Sets the block length (in bytes) for all following block commands
    /// (read and write). Default block length is specified in the CSD.
    ///
    /// ## Arguments:
    /// [31:16] RCA
    /// [15:0] stuff bits
    ///
    /// response type: R1
    SetBlocklen = 16,
    /// ## CMD28
    ///
    /// If the card has write protection features, this command sets the write
    /// protection bit of the addressed group. The properties of write
    /// protection are coded in the card specific data (WP_GRP_SIZE).
    ///
    /// ## Arguments:
    /// [31:0] data address
    ///
    /// response type: R1b
    SetWriteProt = 28,
    /// ## CMD29
    ///
    /// If the card provides write protection features, this command clears
    /// the write protection bit of the addressed group.
    ///
    /// ## Arguments:
    /// [31:0] data address
    ///
    /// response type: R1b
    ClrWriteProt = 29,
    /// ## CMD32
    ///
    /// Sets the address of the first sector of the erase group.
    ///
    /// ## Arguments:
    /// [31:0] data address
    ///
    /// response type: R1
    TagSectorStart = 32,
    /// ## CMD33
    ///
    /// Sets the address of the last sector in a continuous range within the
    /// selection of a single sector to be selected for erase.
    ///
    /// ## Arguments:
    /// [31:0] data address
    ///
    /// response type: R1
    TagSectorEnd = 33,
    /// ## CMD34
    ///
    /// Removes one previously selected sector from the erase selection.
    ///
    /// ## Arguments:
    /// [31:0] data address
    ///
    /// response type: R1
    UntagSector = 34,
    /// ## CMD35
    ///
    /// Sets the address of the first erase group within a range to be
    /// selected for erase.
    ///
    /// ## Arguments:
    /// [31:0] data address
    ///
    /// response type: R1
    TagEraseGroupStart = 35,
    /// ## CMD36
    ///
    /// Sets the address of the last erase group within a continuous range
    /// to be selected for erase.
    ///
    /// ## Arguments:
    /// [31:0] data address
    ///
    /// response type: R1
    TagEraseGroupEnd = 36,
    /// ## CMD37
    ///
    /// Removes one previously selected erase group from the erase selection.
    ///
    /// ## Arguments:
    /// [31:0] data address
    ///
    /// response type: R1
    UntagEraseGroup = 37,
    /// ## CMD38
    ///
    /// Erase all previously selected sectors.
    ///
    /// ## Arguments:
    /// [31:0] stuff bits
    ///
    /// response type: R1b
    Erase = 38,
    /// ## CMD39
    ///
    /// Used to write and read 8-bit (register) data fields. The command
    /// addresses a card, and a register, and provides the data for writing
    /// if the write flag is set. The R4 response contains data read from
    /// the address register. This command accesses application dependent
    /// registers which are not defined in the MMC standard.
    ///
    /// ## Arguments:
    /// [31:16] RCA
    /// [15] register write flag
    /// [14:8] register address
    /// [7:0] register data
    ///
    /// response type: R4
    FastIo = 39,
    /// ## CMD52
    ///
    /// Access a single register within the total 128k of register space in
    /// any I/O function.
    ///
    /// ## Arguments:
    /// [31:0] stuff bits
    ///
    /// response type: R5
    IoRwDirect = 52,
    /// ## CMD53
    ///
    /// Accesses a multiple I/O register with a single command. Allows the
    /// reading or writing of a large number of I/O registers.
    ///
    /// ## Arguments:
    /// [31:0] stuff bits
    ///
    /// response type: R5
    IoRwExtended = 53,
    /// ## CMD55
    ///
    /// Indicates to the card that the next command is an application specific
    /// command rather than a standard command.
    ///
    /// ## Arguments:
    /// [31:16] RCA
    /// [15:0] stuff bits
    ///
    /// response type: R1
    AppCmd = 55,
}

/// Addressed (point-to-point) data transfer commands (adtc)
enum AdtcCommands {
    /// ## CMD6
    ///
    /// # Note:
    ///   CMD6 differs completely between high-speed MMC cards and high-speed SD cards.
    ///   This Command SWITCH_FUNC is for **high-speed SD** cards.
    ///
    /// Checks switch ability (mode 0) and switch card function (mode 1).
    /// Refer to "SD Physical Specification V1.1" for more details.
    ///
    /// ## Arguments:
    /// [31] Mode:  0: Check function 1: Switch function
    /// [30:8] Reserved for function groups 6 ~ 3 (All 0 or 0xFFFF)
    /// [7:4] Function group1 for command system
    /// [3:0] Function group2 for access mode
    ///
    /// response type: R1
    SwitchFunc = 6,
    /// ## CMD8
    ///
    /// The card sends its EXT_CSD register as a block of data, with
    /// a block size of 512 bytes.
    ///
    /// ## Arguments:
    /// [31:0] stuff bits
    ///
    /// response type: R1
    SendExtCsd = 8,
    /// ## CMD11
    ///
    /// Reads data stream from the card, starting at the given address,
    /// until a STOP_TRANSMISSION follows.
    ///
    /// ## Arguments:
    /// [31:0] data address
    ///
    /// response type: R1
    ReadDatUntilStop = 11,
    /// ## CMD17
    ///
    /// Reads a block of the size selected by the SET_BLOCKLEN command.
    ///
    /// ## Arguments:
    /// [31:0] data address
    ///
    /// response type: R1
    ReadSingleBlock = 17,
    /// ## CMD18
    ///
    /// Continuously transfers data blocks from card to host until
    /// interrupted by a stop command.
    ///
    /// ## Arguments:
    /// [31:0] data address
    ///
    /// response type: R1
    ReadMultipleBlock = 18,
    /// ## CMD19
    ///
    /// 64 bytes tuning pattern is sent for SDR50 and SDR104.
    ///
    /// ## Arguments:
    /// [31:0] reserved bits(all 0)
    ///
    /// response type: R1
    SendTuningBlock64 = 19,
    /// ## CMD20
    ///
    /// Writes data stream from the host, starting at the given address,
    /// until a STOP_TRANSMISION follows.
    ///
    /// ## Arguments:
    /// [31:0] data address
    ///
    /// response type: R1
    WriteDatUntilStop = 20,
    /// ## CMD21
    ///
    /// 128 clocks of tuning pattern (64 byte in 4 bit mode or 128 byte in 8 bit mode)
    /// is sent for HS200 optimal sampling point detection.
    ///
    /// ## Arguments:
    /// [31:0] stuff bits
    ///
    /// response type: R1
    SendTuningBlock128 = 21,
    /// ## CMD24
    ///
    /// Writes a block of the size selected by the SET_BLOCKLEN command..
    ///
    /// ## Arguments:
    /// [31:0] data address
    ///
    /// response type: R1
    WriteBlock = 24,
    /// ## CMD25
    ///
    /// Continuously writes blocks of data until a STOP_TRANSMISSION follows.
    ///
    /// ## Arguments:
    /// [31:0] data address
    ///
    /// response type: R1
    WriteMultipleBlock = 25,
    /// ## CMD26
    ///
    /// Programming of the card identification register. This command is issued
    /// only once per card. The card contains hardware to prevent this operation
    /// after the first programming. Normally this command is reserved for the
    /// manufacturer.
    ///
    /// ## Arguments:
    /// [31:0] stuff bits
    ///
    /// response type: R1
    ProgramCid = 26,
    /// ## CMD27
    ///
    /// Programming of the programmable bits of the CSD.
    ///
    /// ## Arguments:
    /// [31:0] stuff bits
    ///
    /// response type: R1
    ProgramCsd = 27,
    /// ## CMD30
    ///
    /// Reads data stream from the card, starting at the given address,
    /// until a STOP_TRANSMISSION follows.
    ///
    /// ## Arguments:
    /// [31:0] write protect data address
    ///
    /// response type: R1
    SendWriteProt = 30,
    /// ## CDM42
    ///
    /// Used to set/reset the password or lock/unlock the card. The size
    /// of the data block is set by the SET_BLOCK_LEN command.
    ///
    /// ## Arguments:
    /// [31:0] stuff bits
    ///
    /// response type: R1b
    LockUnlock = 42,
    /// ## CMD56
    ///
    /// Used either to transfer a data block to the card or to get a data
    /// block from the card for general purpose / application specific
    /// commands. The size of the data block is set by the SET_BLOCK_LEN
    /// command.
    ///
    /// ## Arguments:
    /// [31:1] stuff bits
    /// [0]: RD/WR
    ///
    /// response type: R1b
    GenCmd = 56,
    /// ## CMD60
    ///
    /// These registers are used to control the behavior of the device and
    /// to retrieve status information regarding the operation of the device.
    /// All Status and Control registers are WORD (32-bit) in size and are
    /// WORD aligned. CMD60 is used to read and write these registers.
    ///
    /// ## Arguments:
    /// [31] WR
    /// [30:24] stuff bits
    /// [23:16] address
    /// [15:8] stuff bits
    /// [7:0] byte count
    ///
    /// response type: R1b
    RwMultipleRegister = 60,
    /// ## CMD61
    ///
    /// The host issues a RW_MULTIPLE_BLOCK (CMD61) to begin the data transfer.
    ///
    /// ## Arguments:
    /// [31] WR
    /// [30:16] stuff bits
    /// [15:0] data unit count
    ///
    /// response type: R1b
    RwMultipleBlock = 61,
}

/// ACMDs is preceded with the APP_CMD command. Commands listed are used for SD only,
/// other SD commands not listed are not supported on this module.
enum ACmds {
    /// ## ACMD6
    ///
    /// **Type:** ac
    ///
    /// Defines the data bus width ('00'=1bit or '10'=4bit bus) to be used for
    /// data transfer. The allowed data bus widths are given in SCR register.
    ///
    /// ## Arguments:
    /// [31:2] stuff bits
    /// [1:0] bus width
    ///
    /// response type: R1
    SetBusWidth = 6,
    /// ## ACMD13
    ///
    /// **Type:** adtc
    ///
    /// Send the SD Memory Card status.
    ///
    /// ## Arguments:
    /// [31:0] stuff bits
    ///
    /// response type: R1
    SdStatus = 13,
    /// ## ACMD22
    ///
    /// **Type:** adtc
    ///
    /// Send the number of the written sectors (without errors). Responds
    /// with 32-bit plus the CRC data block.
    ///
    /// ## Arguments:
    /// [31:0] stuff bits
    ///
    /// response type: R1
    SendNumWrSectors = 22,
    /// ## ACMD23
    ///
    /// **Type:** ac
    ///
    /// Set the number of write blocks to be pre-erased before writing
    /// (to be used for fast Multiple Block WR command).
    /// "1" = default(one write block).
    ///
    /// ## Arguments:
    /// [31:23] stuff bits
    /// [22:0] Number of blocks
    ///
    /// response type: R1
    SetWrBlkEraseCount = 23,
    /// ## ACMD41
    ///
    /// **Type:** bcr
    ///
    /// Asks all SDIO cards in idle state to
    /// send them operation conditions
    /// register contents in the response
    /// on the CMD line.
    ///
    /// ## Arguments:
    /// [31:0] OCR
    ///
    /// response type: R3
    SdAppOpCond = 41,
    /// ## ACMD42
    ///
    /// **Type:** ac
    ///
    /// Connect(1)/Disconnect(0) the 50KOhm pull-up resistor on DATA3 of the card.
    ///
    /// ## Arguments:
    /// [31:1] stuff bits
    /// [0] set_cd
    ///
    /// response type: R1
    SetClrCardDetect = 42,
    /// ## ACMD51
    ///
    /// **Type:** adtc
    ///
    /// Reads the SD Configuration Register (SCR).
    ///
    /// ## Arguments:
    /// [31:0] stuff bits
    ///
    /// response type: R1
    SendScr = 51,
}
