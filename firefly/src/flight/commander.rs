use crate::{flight::{arming, modes::FlightMode}, types::{AttitudeTarget, PilotCommand}};

pub struct FlightCommander {
    mode: FlightMode,
}

impl FlightCommander {
    pub const fn new() -> Self {
        Self { mode: FlightMode::Disarmed }
    }

    pub fn mode(&self) -> FlightMode {
        self.mode
    }

    pub fn update(&mut self, cmd: PilotCommand) -> AttitudeTarget {
        self.mode = if arming::armed(cmd) {
            FlightMode::Stabilize
        } else {
            FlightMode::Disarmed
        };

        AttitudeTarget {
            roll_rad: cmd.roll * 0.35,
            pitch_rad: cmd.pitch * 0.35,
            yaw_rate_rads: cmd.yaw * 2.0,
            throttle: cmd.throttle,
        }
    }
}
