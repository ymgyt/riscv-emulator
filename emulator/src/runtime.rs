use thiserror::Error;

use crate::{
    bus::interface::{BusRead, BusWrite},
    cpu::Cpu,
};

#[derive(Error, Debug, PartialEq)]
pub enum RuntimeError {
    #[error("internal error: {message}")]
    Internal { message: String },
}

/// Runtime represents emulator runtime environment.
pub struct Runtime {}

impl Runtime {
    /// Construct `Runtime`
    pub fn new() -> Self {
        Self {}
    }

    /// Entrypoint to run emulator.
    pub fn run<B>(self, bus: B) -> Result<(), RuntimeError>
    where
        B: BusRead + BusWrite,
    {
        let mut cpu = Cpu::new(bus);

        loop {
            if let Err(err) = cpu.cycle() {
                return Err(RuntimeError::Internal {
                    message: format!("{err:#?}"),
                });
            }
            _ = cpu.state();
        }
    }
}

#[cfg(test)]
mod tests {}
