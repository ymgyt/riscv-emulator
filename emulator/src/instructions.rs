use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    /// Load upper immediate
    /// LUI is used to build 32-bit constants and uses the U-type format.
    /// LUI palces the U-immediate value in the top 20bits of the destination register rd
    /// filling in the lowest 12 bits with zeros.
    LUI,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub op_code: OpCode,
    raw: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    U,
}

pub type Immediate = u32;

pub type RegisterIdx = usize;

impl Instruction {
    pub fn format(&self) -> Format {
        use Format::*;
        use OpCode::*;
        match self.op_code {
            LUI => U,
        }
    }

    pub fn imm(&self) -> Immediate {
        match self.format() {
            Format::U => self.raw & 0xffff_f000,
        }
    }

    pub fn rd(&self) -> RegisterIdx {
        let r = (self.raw >> 7) & 0x1f;
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
            0b0110111 => LUI,

            _ => return Err(DecodeError::InvalidOpCode),
        };
        Ok(Instruction {
            op_code,
            raw: instruction,
        })
    }
}
