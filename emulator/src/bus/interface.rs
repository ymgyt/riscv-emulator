use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum BusReadException {
    #[error("load address misaligned")]
    LoadAddressMisaligned,
    #[error("load access fault")]
    LoadAccessFault,
}

pub trait BusRead {
    fn read8(&self, addr: u32) -> Result<u8, BusReadException>;
    fn read16(&self, addr: u32) -> Result<u16, BusReadException>;
    fn read32(&self, addr: u32) -> Result<u32, BusReadException>;
}

#[derive(Error, Debug, Clone, Copy)]
pub enum BusWriteException {
    #[error("store address misaligned")]
    StoreAddressMisaligned,
    #[error("store acces fault")]
    StoreAccessFault,
}

pub trait BusWrite {
    fn write8(&mut self, addr: u32, v: u8) -> Result<(), BusWriteException>;
    fn write16(&mut self, addr: u32, v: u16) -> Result<(), BusWriteException>;
    fn write32(&mut self, addr: u32, v: u32) -> Result<(), BusWriteException>;
}
