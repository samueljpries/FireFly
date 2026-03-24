use crate::types::BatterySample;

pub trait BatteryMonitor {
    fn read(&mut self) -> BatterySample;
}

pub struct DummyBattery;

impl DummyBattery {
    pub const fn new() -> Self {
        Self
    }
}

impl BatteryMonitor for DummyBattery {
    fn read(&mut self) -> BatterySample {
        BatterySample {
            voltage_v: 11.8,
            current_a: 4.2,
        }
    }
}
