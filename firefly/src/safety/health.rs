use crate::types::{HealthState, SensorSnapshot};

pub struct HealthMonitor {
    low_voltage_v: f32,
    critical_voltage_v: f32,
}

impl HealthMonitor {
    pub const fn new(low_voltage_v: f32, critical_voltage_v: f32) -> Self {
        Self {
            low_voltage_v,
            critical_voltage_v,
        }
    }

    pub fn evaluate(&self, sensors: SensorSnapshot) -> HealthState {
        if sensors.battery.voltage_v <= self.critical_voltage_v || !sensors.rc_link_ok {
            HealthState::Fault
        } else if sensors.battery.voltage_v <= self.low_voltage_v {
            HealthState::Degraded
        } else {
            HealthState::Ok
        }
    }
}
