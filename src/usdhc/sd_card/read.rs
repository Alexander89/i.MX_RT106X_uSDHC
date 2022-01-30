use super::SdCard;

impl SdCard {
    pub fn read_block(&mut self, buffer: &[u8], size: usize) -> Result<usize, ()> {
        // The CMD17, CMD18, CMD53, CMD60, CMD61, and so on, can initiate a block read

        Ok(size)
    }
}
