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
    /// The jump and link (JAL) instruction uses the J-type format,
    /// where the J-immediate encodes a signed offset in multiples of 2 bytes.
    /// The offset is sign-extended and added to the address of the jump instruction to form the jump target address.
    /// Jumps can therefore target a ±1 MiB range. JAL stores the address of the instruction following the jump (pc+4) into register rd.
    Jal,
    /// The indirect jump instruction JALR (jump and link register) uses the I-type encoding.
    /// The target address is obtained by adding the sign-extended 12-bit I-immediate to the register rs1
    /// then setting the least-significant bit of the result to zero.
    /// The address of the instruction following the jump (pc+4) is written to register rd.
    Jalr,
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
    I,
}

pub type RegisterIdx = usize;

impl Instruction {
    pub fn format(&self) -> Format {
        use Format::*;
        use OpCode::*;
        match self.op_code {
            Lui | Auipc => U,
            Jal => J,
            Jalr => I,
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
            Format::I => {
                let imm = self.ir >> 20;
                let imm = if imm & 0x800 != 0 {
                    imm | 0xfffff000
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

    pub fn rs1(&self) -> usize {
        let r = (self.ir >> 15) & 0x1f;
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
            0b1100111 => Jalr,

            _ => return Err(DecodeError::InvalidOpCode),
        };
        Ok(Instruction {
            op_code,
            ir: instruction,
        })
    }
}
