use super::interface::{BusRead, BusReadException};

pub(crate) struct Bus {}

impl Bus {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl BusRead for Bus {
    fn read8(&self, addr: u32) -> Result<u8, BusReadException> {
        Ok(0)
    }
    fn read16(&self, addr: u32) -> Result<u16, BusReadException> {
        Ok(0)
    }
    fn read32(&self, addr: u32) -> Result<u32, BusReadException> {
        Ok(0)
    }
}
