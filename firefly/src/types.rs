#[derive(Clone, Copy, Debug, Default)]
pub struct ImuSample {
    pub accel_mps2: [f32; 3],
    pub gyro_rads: [f32; 3],
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PowerState {
    pub battery_v: f32,
    pub current_a: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PilotCommand {
    pub throttle: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub arm: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MotorCommand {
    pub m1: f32,
    pub m2: f32,
    pub m3: f32,
    pub m4: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct VehicleState {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub roll_rate: f32,
    pub pitch_rate: f32,
    pub yaw_rate: f32,
    pub altitude_m: f32,
    pub battery_v: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AttitudeTarget {
    pub roll_rad: f32,
    pub pitch_rad: f32,
    pub yaw_rate_rads: f32,
    pub throttle: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RateTarget {
    pub roll_rate_rads: f32,
    pub pitch_rate_rads: f32,
    pub yaw_rate_rads: f32,
    pub throttle: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BatterySnapshot {
    pub voltage_v: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SensorSnapshot {
    pub imu: ImuSample,
    pub battery: BatterySnapshot,
    pub rc_link_ok: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HealthState {
    Ok,
    Degraded,
    Fault,
}

impl Default for HealthState {
    fn default() -> Self {
        Self::Ok
    }
}