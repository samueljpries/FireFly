use crate::types::ImuSample;

pub trait ImuDriver {
    fn read_sample(&mut self) -> Result<ImuSample, ImuError>;
}

#[derive(Debug)]
pub enum ImuError {
    ReadError,
}

pub struct DummyImu;

impl ImuDriver for DummyImu {
    fn read_sample(&mut self) -> Result<ImuSample, ImuError> {
        Ok(ImuSample::default())
    }
}