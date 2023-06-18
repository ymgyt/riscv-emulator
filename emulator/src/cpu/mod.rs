mod macros;
use macros::add_imm_signed;

use thiserror::Error;

use crate::{
    bus::interface::{BusRead, BusReadException},
    instructions::{DecodeError, Decoder, Instruction, OpCode, RegisterIdx},
};

#[derive(Debug)]
pub struct Cpu<B> {
    bus: B,
    state: State,
    r: Registers,
    decoder: Decoder,
}

#[derive(Debug)]
pub struct State {
    pub cycle_counter: u64,
}

#[derive(Debug)]
struct Registers {
    /// Program counter
    pc: u32,
    x: [u32; 32],
}

impl<B> Cpu<B> {
    pub fn new(bus: B) -> Self {
        Self {
            bus,
            state: State { cycle_counter: 0 },
            r: Registers { pc: 0, x: [0; 32] },
            decoder: Decoder::new(),
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}

#[derive(Error, Debug)]
pub enum CpuError {
    #[error("load error {0:?}")]
    Load(#[from] BusReadException),
    #[error("decode error: {0:?}")]
    Decode(DecodeError),
}

#[derive(Debug)]
enum Effect<B> {
    UpdateRegister {
        rd: RegisterIdx,
        imm: u32,
    },
    Jal {
        rd: RegisterIdx,
        pc: u32,
        imm: i32,
    },
    Jalr {
        rd: RegisterIdx,
        pc: u32,
        offset: i32,
        base: u32,
    },
    Branch {
        do_branch: bool,
        pc: u32,
        imm: i32,
    },
    Load {
        effective_addr: u32,
        rd: RegisterIdx,
        load: fn(u32, &B) -> Result<u32, BusReadException>,
    },
}

impl<B> Cpu<B>
where
    B: BusRead,
{
    /// Emulate cpu clock cycle.
    /// Decode instruction from pc.
    /// Process instruction and update state.
    pub fn cycle(&mut self) -> Result<(), CpuError> {
        self.state.cycle_counter = self.state.cycle_counter.wrapping_add(1);

        self.next_instruction()
            .and_then(|ir| self.process(ir))
            .and_then(|effect| self.apply(effect))
    }

    /// Read and decode next instruction.
    fn next_instruction(&self) -> Result<Instruction, CpuError> {
        let ir = self.bus.read32(self.r.pc).map_err(CpuError::Load)?;
        self.decoder.try_decode(ir).map_err(CpuError::Decode)
    }

    /// Return side effects resulting from processing instruction.
    fn process(&self, ir: Instruction) -> Result<Effect<B>, CpuError> {
        use OpCode::*;
        let effect = match ir.op_code {
            Lui => Effect::UpdateRegister {
                rd: ir.rd(),
                imm: ir.imm(),
            },
            Auipc => Effect::UpdateRegister {
                rd: ir.rd(),
                imm: ir.imm() + self.r.pc,
            },
            Jal => Effect::Jal {
                rd: ir.rd(),
                imm: ir.imm_signed(),
                pc: self.r.pc,
            },
            Jalr => Effect::Jalr {
                rd: ir.rd(),
                pc: self.r.pc,
                offset: ir.imm_signed(),
                base: self.read(ir.rs1()),
            },
            Beq => self.branch_with_unsigned(|r1, r2| r1 == r2, ir),
            Bne => self.branch_with_unsigned(|r1, r2| r1 != r2, ir),
            Bltu => self.branch_with_unsigned(|r1, r2| r1 < r2, ir),
            Bgeu => self.branch_with_unsigned(|r1, r2| r1 >= r2, ir),
            Blt => self.branch_with_signed(|r1, r2| r1 < r2, ir),
            Bge => self.branch_with_signed(|r1, r2| r1 >= r2, ir),
            Lb => self.load_with(|addr, bus| bus.read8(addr).map(|v| v as i8 as u32), ir),
            Lh => self.load_with(|addr, bus| bus.read16(addr).map(|v| v as i16 as u32), ir),
            Lw => self.load_with(|addr, bus| bus.read32(addr), ir),
            Lbu => self.load_with(|addr, bus| bus.read8(addr).map(|v| v as u32), ir),
            Lhu => self.load_with(|addr, bus| bus.read16(addr).map(|v| v as u32), ir),
        };
        Ok(effect)
    }

    /// Apply side effect to update state.
    fn apply(&mut self, effect: Effect<B>) -> Result<(), CpuError> {
        use Effect::*;
        let do_inc = match effect {
            UpdateRegister { rd, imm } => {
                self.write(rd, imm);
                true
            }
            Jal { rd, pc, imm } => {
                self.write(rd, pc + 4);
                self.r.pc = (pc as i64 + imm as i64) as u32;
                false
            }
            Jalr {
                rd,
                pc,
                offset,
                base,
            } => {
                self.write(rd, pc + 4);
                let target = (base as i64 + offset as i64) as u32;
                self.r.pc = target & !1;
                false
            }
            Branch { do_branch, pc, imm } => do_branch
                .then(|| {
                    self.r.pc = (pc as i64 + imm as i64) as u32;
                })
                .is_none(),
            Load {
                effective_addr,
                rd,
                load,
            } => {
                let v = load(effective_addr, &self.bus)?;
                self.write(rd, v);
                true
            }
        };

        do_inc.then(|| self.r.pc += 4);

        Ok(())
    }

    fn branch_with_unsigned<F: Fn(u32, u32) -> bool>(&self, f: F, ir: Instruction) -> Effect<B> {
        let do_branch = f(self.read(ir.rs1()), self.read(ir.rs2()));

        Effect::Branch {
            do_branch,
            pc: self.r.pc,
            imm: ir.imm_signed(),
        }
    }

    fn branch_with_signed<F: Fn(i32, i32) -> bool>(&self, f: F, ir: Instruction) -> Effect<B> {
        let do_branch = f(self.read(ir.rs1()) as i32, self.read(ir.rs2()) as i32);

        Effect::Branch {
            do_branch,
            pc: self.r.pc,
            imm: ir.imm_signed(),
        }
    }

    fn load_with(
        &self,
        load: fn(u32, &B) -> Result<u32, BusReadException>,
        ir: Instruction,
    ) -> Effect<B> {
        let effective_addr = add_imm_signed!(ir.rs1(), ir.imm_signed());
        Effect::Load {
            effective_addr,
            rd: ir.rd(),
            load,
        }
    }

    /// Write value to rd register
    /// Write to x0 register are ignored
    fn write(&mut self, rd: usize, v: u32) {
        if rd != 0 {
            self.r.x[rd] = v;
        }
    }

    /// Read from register
    fn read(&self, rs: usize) -> u32 {
        self.r.x[rs]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::Bus;

    #[test]
    fn should_increment_cycle_counter() {
        let bus = Bus::new(vec![0; 1024]);
        let mut c = Cpu::new(bus);
        c.cycle().ok();
        assert_eq!(c.state().cycle_counter, 1);
    }

    #[test]
    fn instruction_lui() {
        let lui: u32 = (0b1 << 12) | (0b01 << 7) | 0b00110111;
        let ram = lui.to_le_bytes().into();
        let bus = Bus::new(ram);
        let mut c = Cpu::new(bus);
        c.cycle().unwrap();
        // LUI filling in the lowest 12 bits with zeros.
        assert_eq!(c.r.x[1], 4096);
    }

    #[test]
    fn instruction_auipc() {
        // Since this instruction implicitly performs an addition to the current pc
        // it is preferable to have an initial pc other than 0
        let lui: u32 = (0b1 << 12) | (0b01 << 7) | 0b0010111;
        let ram = lui.to_le_bytes().into();
        let bus = Bus::new(ram);
        let mut c = Cpu::new(bus);
        c.cycle().unwrap();
        assert_eq!(c.r.x[1], 4096);
    }
}