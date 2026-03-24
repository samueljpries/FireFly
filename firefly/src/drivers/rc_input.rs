use crate::types::PilotCommand;

pub trait RcInput {
    fn read(&mut self) -> Result<PilotCommand, RcInputError>;
}

#[derive(Debug)]
pub enum RcInputError {
    ReadError,
}

pub struct DummyRcInput;

impl RcInput for DummyRcInput {
    fn read(&mut self) -> Result<PilotCommand, RcInputError> {
        Ok(PilotCommand::default())
    }
}