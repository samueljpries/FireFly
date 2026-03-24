use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};

use crate::drivers::motor_output::DummyMotorOutput;
use crate::drivers::motor_output::MotorOutput;
use crate::types::MotorCommand;

#[embassy_executor::task]
pub async fn run(
    mut motors: DummyMotorOutput,
    motor_command: &'static Mutex<CriticalSectionRawMutex, MotorCommand>,
) {
    loop {
        let cmd = {
            let guard = motor_command.lock().await;
            *guard
        };

        let _ = motors.write(cmd);

        Timer::after(Duration::from_millis(4)).await;
    }
}