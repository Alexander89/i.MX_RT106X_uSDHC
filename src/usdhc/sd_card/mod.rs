mod read;

pub struct SdCard {
    max_block_size: usize,
}

impl SdCard {
    pub fn send_command(&mut self) -> Result<(), ()> {
        Ok(())
    }

    pub fn is_ready_for_data(&mut self) -> Result<(), ()> {
        // ral::usdhc::DLL_STATUS
        Ok(())
    }
}
