use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    /// Load upper immediate
    /// LUI is used to build 32-bit constants and uses the U-type format.
    /// LUI palces the U-immediate value in the top 20bits of the destination register rd
    /// filling in the lowest 12 bits with zeros.
    Lui,
    /// Add upper immediate to pc
    /// Auipc is used to build pc-relative addressed and uses the U-type format.
    /// AUIPC forms a 32-bit offset from the 20-bit U-immediaate, filling in the lowest 12 bits with zeros
    /// adds this offset to the address of the AUIPC instruction, then places the result in register rd.
    Auipc,
    Jal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub op_code: OpCode,
    ir: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    U,
    J,
}

pub type RegisterIdx = usize;

impl Instruction {
    pub fn format(&self) -> Format {
        use Format::*;
        use OpCode::*;
        match self.op_code {
            Lui | Auipc => U,
            Jal => J,
        }
    }

    pub fn imm(&self) -> u32 {
        match self.format() {
            Format::U => self.ir & 0xffff_f000,
            _ => todo!(),
        }
    }

    pub fn imm_signed(&self) -> i32 {
        match self.format() {
            Format::J => {
                let imm = ((self.ir & 0x80000000) >> 11)
                    | ((self.ir & 0x7fe00000) >> 20)
                    | ((self.ir & 0x00100000) >> 9)
                    | (self.ir & 0x000ff000);

                let imm = if imm & 0x0010_0000 != 0 {
                    imm | 0xffe0_0000
                } else {
                    imm
                };
                imm as i32
            }
            _ => unreachable!(),
        }
    }

    pub fn rd(&self) -> RegisterIdx {
        let r = (self.ir >> 7) & 0x1f;
        r as usize
    }
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("invalid opcode")]
    InvalidOpCode,
}

#[derive(Debug)]
pub struct Decoder {}

impl Decoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn try_decode(&self, instruction: u32) -> Result<Instruction, DecodeError> {
        use OpCode::*;
        // Volume I: RISC-V Unprivileged ISA V20191213 P130
        let op_code = match instruction & 0x7f {
            0b0110111 => Lui,
            0b0010111 => Auipc,
            0b1101111 => Jal,

            _ => return Err(DecodeError::InvalidOpCode),
        };
        Ok(Instruction {
            op_code,
            ir: instruction,
        })
    }
}
