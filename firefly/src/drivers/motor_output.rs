use crate::types::MotorCommand;

pub trait MotorOutput {
    fn write(&mut self, cmd: MotorCommand);
}

pub struct DummyMotorOutput {
    pub last: MotorCommand,
}

impl DummyMotorOutput {
    pub const fn new() -> Self {
        Self {
            last: MotorCommand {
                m1: 0.0,
                m2: 0.0,
                m3: 0.0,
                m4: 0.0,
            },
        }
    }
}

impl MotorOutput for DummyMotorOutput {
    fn write(&mut self, cmd: MotorCommand) {
        self.last = cmd;
    }
}
