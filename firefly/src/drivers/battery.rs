use crate::types::PowerState;

pub trait BatteryMonitor {
    fn read(&mut self) -> Result<PowerState, BatteryError>;
}

#[derive(Debug)]
pub enum BatteryError {
    ReadError,
}

pub struct DummyBatteryMonitor;

impl BatteryMonitor for DummyBatteryMonitor {
    fn read(&mut self) -> Result<PowerState, BatteryError> {
        Ok(PowerState::default())
    }
}