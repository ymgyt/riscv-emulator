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

    /// All branch instructions use the B-type instruction format.
    /// The 12-bit B-immediate encodes signed offsets in multiples of 2 bytes.
    /// The offset is sign-extended and added to the address of the branch instruction to give the target address.
    /// The conditional branch range is ±4 KiB.
    /// Branch if Equal r1 == r2
    Beq,
    /// Branch if r1 != r2
    Bne,
    /// Branch if r1 < r2
    Blt,
    Bltu,
    /// Branch if r1 >= r2
    Bge,
    Bgeu,

    /// Load and store instructions transfer a value between the registers and memory.
    /// Loads are encoded in the I-type format and stores are S-type.
    /// The effective address is obtained by adding register rs1 to the sign-extended 12-bit offset.
    /// Loads copy a value from memory to register rd. Stores copy the value in register rs2 to memory.
    /// Load byte
    Lb,
    /// Load halfword
    Lh,
    /// Load word
    Lw,
    /// Load byte unsigned
    Lbu,
    /// Load halfword unsigned
    Lhu,

    /// Atomic read/write csr
    Csrrw,
    /// Atomic read and set bits
    Csrrs,
    Csrrc,
    Csrrwi,
    Csrrsi,
    Csrrci,
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
    B,
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
            Beq | Bne | Blt | Bltu | Bge | Bgeu => B,
            Lb | Lh | Lw | Lbu | Lhu => I,
            Csrrw | Csrrs | Csrrc | Csrrwi | Csrrsi | Csrrci => I,
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
            Format::B => {
                let imm = ((self.ir & 0x80000000) >> 19)
                    | ((self.ir & 0x7e000000) >> 20)
                    | ((self.ir & 0x00000800) >> 7)
                    | ((self.ir & 0x00000080) << 4);
                let imm = if imm & 0x1000 != 0 {
                    imm | 0xffffe000
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

    pub fn rs2(&self) -> usize {
        let r = (self.ir >> 20) & 0x1f;
        r as usize
    }

    /// for CSR Instructions
    pub fn csr(&self) -> usize {
        // make sure instruction is csr
        debug_assert_eq!(self.ir & 0x7f, 0b1110011);
        let r = self.ir >> 20;
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
            0b1100011 => match (instruction >> 12) & 0x07 {
                0b000 => Beq,
                0b001 => Bne,
                0b100 => Blt,
                0b101 => Bge,
                0b110 => Bltu,
                0b111 => Bgeu,
                _ => return Err(DecodeError::InvalidOpCode),
            },
            0b0000011 => match (instruction >> 12) & 0x07 {
                0b000 => Lb,
                0b001 => Lh,
                0b010 => Lw,
                0b100 => Lbu,
                0b101 => Lhu,
                _ => return Err(DecodeError::InvalidOpCode),
            },
            0b1110011 => match (instruction >> 12) & 0x07 {
                0b001 => Csrrw,
                0b010 => Csrrs,
                0b011 => Csrrc,
                0b101 => Csrrwi,
                0b110 => Csrrsi,
                0b111 => Csrrci,
                _ => return Err(DecodeError::InvalidOpCode),
            },

            _ => return Err(DecodeError::InvalidOpCode),
        };
        Ok(Instruction {
            op_code,
            ir: instruction,
        })
    }
}
