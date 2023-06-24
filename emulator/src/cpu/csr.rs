use crate::instructions::RegisterIdx;

enum CsrAddr {
    Mstatus = 0x300,
}

/// Control and Status Register
#[derive(Debug)]
pub struct Csr {
    r: [u32; Self::ADDR_SPACE],
}

impl Csr {
    const ADDR_SPACE: usize = 4096;
    pub fn new() -> Self {
        Self {
            r: [0; Self::ADDR_SPACE],
        }
    }

    pub fn read_mstatus(&self) -> Mstatus {
        Mstatus(self.read(CsrAddr::Mstatus as usize))
    }

    pub fn read(&self, addr: RegisterIdx) -> u32 {
        self.r[addr]
    }

    pub fn write(&mut self, addr: RegisterIdx, value: u32) {
        self.r[addr] = value;
    }
}

pub struct Mstatus(u32);

impl Mstatus {
    /// Return machine interrupt enable bit
    pub fn mie(&self) -> bool {
        (self.0 & 0x08) != 0
    }
}
