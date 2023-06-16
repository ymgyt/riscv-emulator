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
    #[error("load error: {0:?}")]
    Load(BusReadException),
    #[error("decode error: {0:?}")]
    Decode(DecodeError),
}

#[derive(Debug)]
enum Effect {
    UpdateRegister { rd: RegisterIdx, imm: u32 },
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
        println!("{ir:#b}");
        self.decoder.try_decode(ir).map_err(CpuError::Decode)
    }

    /// Return side effects resulting from processing instruction.
    fn process(&self, ir: Instruction) -> Result<Effect, CpuError> {
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
        };
        Ok(effect)
    }

    /// Apply side effect to update state.
    fn apply(&mut self, effect: Effect) -> Result<(), CpuError> {
        use Effect::*;
        println!("{effect:?}");
        match effect {
            UpdateRegister { rd, imm } => {
                self.update_register(rd, imm);
            }
        }
        Ok(())
    }

    fn update_register(&mut self, rd: RegisterIdx, v: u32) {
        // Register x0 is always zero
        if rd != 0 {
            self.r.x[rd] = v;
        }
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
