use crate::types::PilotCommand;

pub trait RcInput {
    fn read(&mut self) -> PilotCommand;
    fn link_ok(&self) -> bool;
}

pub struct DummyRcInput {
    link_ok: bool,
}

impl DummyRcInput {
    pub const fn new() -> Self {
        Self { link_ok: true }
    }
}

impl RcInput for DummyRcInput {
    fn read(&mut self) -> PilotCommand {
        PilotCommand {
            throttle: 0.15,
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
            arm: false,
        }
    }

    fn link_ok(&self) -> bool {
        self.link_ok
    }
}
