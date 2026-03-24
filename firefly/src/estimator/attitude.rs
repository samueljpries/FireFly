use crate::types::{ImuSample, VehicleState};
use libm::{atan2f, sqrtf};

pub struct ComplementaryAttitudeEstimator {
    alpha: f32,
    roll: f32,
    pitch: f32,
    yaw: f32,
}

impl ComplementaryAttitudeEstimator {
    pub const fn new(alpha: f32) -> Self {
        Self {
            alpha,
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }

    pub fn update(&mut self, sample: ImuSample, battery_v: f32, dt_s: f32) -> VehicleState {
        self.roll += sample.gyro_rads[0] * dt_s;
        self.pitch += sample.gyro_rads[1] * dt_s;
        self.yaw += sample.gyro_rads[2] * dt_s;

        let accel_roll = atan2f(sample.accel_mps2[1], sample.accel_mps2[2]);
        let accel_pitch = atan2f(-sample.accel_mps2[0], sqrtf(sample.accel_mps2[1] * sample.accel_mps2[1] + sample.accel_mps2[2] * sample.accel_mps2[2])
    );

        self.roll = self.alpha * self.roll + (1.0 - self.alpha) * accel_roll;
        self.pitch = self.alpha * self.pitch + (1.0 - self.alpha) * accel_pitch;

        VehicleState {
            roll_rad: self.roll,
            pitch_rad: self.pitch,
            yaw_rad: self.yaw,
            roll_rate_rads: sample.gyro_rads[0],
            pitch_rate_rads: sample.gyro_rads[1],
            yaw_rate_rads: sample.gyro_rads[2],
            battery_v,
        }
    }
}
