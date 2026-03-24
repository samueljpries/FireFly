use crate::types::MotorCommand;

pub trait MotorOutput {
    fn write(&mut self, cmd: MotorCommand) -> Result<(), MotorError>;
}

#[derive(Debug)]
pub enum MotorError {
    WriteError,
}

pub struct DummyMotorOutput;

impl MotorOutput for DummyMotorOutput {
    fn write(&mut self, _cmd: MotorCommand) -> Result<(), MotorError> {
        Ok(())
    }
}