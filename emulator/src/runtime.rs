use thiserror::Error;

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
    pub fn run(self) -> Result<(), RuntimeError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_run() {
        let r = Runtime::new();
        assert_eq!(r.run(), Ok(()));
    }
}
