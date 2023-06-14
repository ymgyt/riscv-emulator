use crate::bus::interface::{BusRead, BusReadException};

#[derive(Debug)]
pub struct Cpu<B> {
    bus: B,
    state: State,
    r: Registers,
}

#[derive(Debug)]
pub struct State {
    pub cycle_counter: u64,
}

#[derive(Debug)]
struct Registers {
    /// Program counter
    pc: u32,
}

impl<B> Cpu<B> {
    pub fn new(bus: B) -> Self {
        Self {
            bus,
            state: State { cycle_counter: 0 },
            r: Registers { pc: 0 },
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}

impl<B> Cpu<B>
where
    B: BusRead,
{
    pub fn cycle(&mut self) {
        self.state.cycle_counter = self.state.cycle_counter.wrapping_add(1);

        let _ir = match self.read_instruction() {
            Ok(ir) => ir,
            Err(_) => todo!(),
        };
    }

    fn read_instruction(&self) -> Result<u32, BusReadException> {
        self.bus.read32(self.r.pc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::imp::Bus;

    #[test]
    fn should_increment_cycle_counter() {
        let bus = Bus::new();
        let mut c = Cpu::new(bus);
        c.cycle();
        assert_eq!(c.state().cycle_counter, 1);
    }
}
