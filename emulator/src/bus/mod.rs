pub(crate) mod interface;

use interface::{BusRead, BusReadException};

pub(crate) struct Bus {
    ram: Vec<u8>,
}

impl Bus {
    pub(crate) fn new(ram: Vec<u8>) -> Self {
        Self { ram }
    }
}

// TODO: alignment check, address check
impl BusRead for Bus {
    fn read8(&self, _addr: u32) -> Result<u8, BusReadException> {
        todo!()
    }
    fn read16(&self, _addr: u32) -> Result<u16, BusReadException> {
        todo!()
    }
    fn read32(&self, addr: u32) -> Result<u32, BusReadException> {
        if addr & 3 != 0 {
            Err(BusReadException::LoadAddressMisaligned)
        } else {
            let addr = addr as usize;
            Ok(u32::from_le_bytes([
                self.ram[addr],
                self.ram[addr + 1],
                self.ram[addr + 2],
                self.ram[addr + 3],
            ]))
        }
    }
}
