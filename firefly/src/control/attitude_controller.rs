use crate::{
    control::pid::Pid,
    types::{AttitudeTarget, RateTarget, VehicleState},
};

pub struct AttitudeController {
    roll: Pid,
    pitch: Pid,
}

impl Default for AttitudeController {
    fn default() -> Self {
        Self {
            roll: Pid::new(4.0, 0.0, 0.2, -4.0, 4.0),
            pitch: Pid::new(4.0, 0.0, 0.2, -4.0, 4.0),
        }
    }
}

impl AttitudeController {
    pub fn update(&mut self, target: AttitudeTarget, state: VehicleState, dt: f32) -> RateTarget {
        RateTarget {
            roll_rate_rads: self.roll.update(target.roll_rad, state.roll, dt),
            pitch_rate_rads: self.pitch.update(target.pitch_rad, state.pitch, dt),
            yaw_rate_rads: target.yaw_rate_rads,
            throttle: target.throttle,
        }
    }
}
