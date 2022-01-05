pub struct DataTransfer<'data> {
    data: &'data [u8],
    length: usize,
    max_block_size: usize,
}

impl<'data> DataTransfer<'data> {
    pub fn new(data: &'data [u8], length: usize, max_block_size: usize) -> Self {
        Self {
            data,
            length,
            max_block_size,
        }
    }

    fn send_data(&mut self) {
        let data_lng = self.data.len();
        let large_block_count = data_lng / self.max_block_size;

        let chunks = self.data.chunks(self.max_block_size);
        let last_lng = 0;
        for c in chunks.into_iter() {
            let lng = c.len();
            if lng != last_lng {
                if lng == self.max_block_size {
                    // send new CMD 53 + large_block_count
                } else {
                    // send new CMD 53 + 1 block
                }
            }
            // send Block data (c)
        }
    }

    fn calc_cmd_crc() -> u8 {
        // The CRC polynomials for the CMD are as follows:
        //
        // Generator polynomial: G(x) = x^7 + x^3 + 1
        // M(x) = (first bit) * x^n + (second bit) * x^(n-1) +...+ (last bit) * x^0
        // CRC[6:0] = Remainder [(M(x) * x^7) / G(x)]

        //7bit
        0
    }
    fn calc_data_crc() -> u16 {
        // The CRC polynomials for the data are as follows:
        //
        // Generator polynomial: G(x) = x^16 + x^12 + x^5 + 1
        // M(x) = (first bit) * x^n + (second bit) * x^(n-1) +...+ (last bit) * x^0
        // CRC[15:0] = Remainder [(M(x) * x^16) / G(x)]
        0
    }
}

/// The access bits for the EXT_CSD access modes are listed in the following enumeration.
pub enum ExtCsd {
    /// The command set is changed according to the Cmd Set field of the argument.
    CommandSet = 0b00,
    /// The bits in the pointed byte are set, according to the bits set to 1 in the Value field.
    SetBits = 0b01,
    /// The bits in the pointed byte are cleared, according to the bits set to 1 in the Value field.
    ClearBits = 0b10,
    /// The Value field is written into the pointed byte.
    WriteByte = 0b11,
}
