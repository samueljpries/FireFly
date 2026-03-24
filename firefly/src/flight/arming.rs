use crate::types::PilotCommand;

pub fn armed(cmd: PilotCommand) -> bool {
    cmd.arm && cmd.throttle < 0.05
}
