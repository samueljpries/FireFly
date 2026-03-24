#[derive(Clone, Copy, Debug)]
pub struct LoopRates {
    pub sensor_hz: u16,
    pub control_hz: u16,
    pub telemetry_hz: u16,
    pub safety_hz: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct MotorLimits {
    pub min_output: f32,
    pub idle_output: f32,
    pub max_output: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct BatteryConfig {
    pub low_voltage_v: f32,
    pub critical_voltage_v: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct AppConfig {
    pub rates: LoopRates,
    pub motors: MotorLimits,
    pub battery: BatteryConfig,
}

pub const APP_CONFIG: AppConfig = AppConfig {
    rates: LoopRates {
        sensor_hz: 500,
        control_hz: 250,
        telemetry_hz: 10,
        safety_hz: 50,
    },
    motors: MotorLimits {
        min_output: 0.0,
        idle_output: 0.08,
        max_output: 1.0,
    },
    battery: BatteryConfig {
        low_voltage_v: 10.5,
        critical_voltage_v: 9.6,
    },
};
