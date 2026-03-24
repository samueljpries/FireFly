use crate::types::ImuSample;

#[derive(Debug, Clone, Copy)]
pub enum ImuError {
    Bus,
    InvalidData,
}

pub trait ImuDriver {
    fn init(&mut self) -> Result<(), ImuError>;
    fn read_sample(&mut self) -> Result<ImuSample, ImuError>;
}

pub struct DummyImu;

impl DummyImu {
    pub const fn new() -> Self {
        Self
    }
}

impl ImuDriver for DummyImu {
    fn init(&mut self) -> Result<(), ImuError> {
        Ok(())
    }

    fn read_sample(&mut self) -> Result<ImuSample, ImuError> {
        Ok(ImuSample {
            accel_mps2: [0.0, 0.0, 9.81],
            gyro_rads: [0.0, 0.0, 0.0],
        })
    }
}
