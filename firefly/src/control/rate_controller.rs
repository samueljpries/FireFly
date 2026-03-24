use crate::{control::pid::Pid, types::{MotorCommand, RateTarget, VehicleState}};

pub struct RateController {
    roll: Pid,
    pitch: Pid,
    yaw: Pid,
}

impl Default for RateController {
    fn default() -> Self {
        Self {
            roll: Pid::new(0.25, 0.10, 0.002, -1.0, 1.0),
            pitch: Pid::new(0.25, 0.10, 0.002, -1.0, 1.0),
            yaw: Pid::new(0.20, 0.05, 0.0, -1.0, 1.0),
        }
    }
}

impl RateController {
    pub fn update(&mut self, target: RateTarget, state: VehicleState, dt: f32) -> MotorCommand {
        let roll = self.roll.update(target.roll_rate_rads, state.roll_rate, dt);
        let pitch = self.pitch.update(target.pitch_rate_rads, state.pitch_rate, dt);
        let yaw = self.yaw.update(target.yaw_rate_rads, state.yaw_rate, dt);

        MotorCommand {
            m1: target.throttle + roll - pitch + yaw,
            m2: target.throttle - roll - pitch - yaw,
            m3: target.throttle - roll + pitch + yaw,
            m4: target.throttle + roll + pitch - yaw,
        }
    }
}
