use crate::types::MotorCommand;

pub struct QuadXMix {
    min: f32,
    max: f32,
}

impl QuadXMix {
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn apply(&self, mut cmd: MotorCommand, armed: bool) -> MotorCommand {
        if !armed {
            return MotorCommand::default();
        }
        cmd.m1 = cmd.m1.clamp(self.min, self.max);
        cmd.m2 = cmd.m2.clamp(self.min, self.max);
        cmd.m3 = cmd.m3.clamp(self.min, self.max);
        cmd.m4 = cmd.m4.clamp(self.min, self.max);
        cmd
    }
}
