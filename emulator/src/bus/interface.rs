#[derive(Debug, Clone, Copy)]
pub enum BusReadException {
    LoadAddressMisaligned,
    LoadAccessFault,
}

pub trait BusRead {
    fn read8(&self, addr: u32) -> Result<u8, BusReadException>;
    fn read16(&self, addr: u32) -> Result<u16, BusReadException>;
    fn read32(&self, addr: u32) -> Result<u32, BusReadException>;
}

#[derive(Debug, Clone, Copy)]
pub enum BusWriteException {
    StoreAddressMisaligned,
    StoreAccessFault,
}

pub trait BusWrite {
    fn write8(&mut self, addr: u32, v: u8) -> Result<(), BusWriteException>;
    fn write16(&mut self, addr: u32, v: u16) -> Result<(), BusWriteException>;
    fn write32(&mut self, addr: u32, v: u32) -> Result<(), BusWriteException>;
}
