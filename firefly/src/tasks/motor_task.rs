use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::{Duration, Timer};

use crate::{config::APP_CONFIG, drivers::motor_output::MotorOutput, types::MotorCommand};

#[embassy_executor::task]
pub async fn run<M>(
    mut motors: M,
    motor_command: &'static Mutex<CriticalSectionRawMutex, MotorCommand>,
) where
    M: MotorOutput + 'static,
{
    let period_ms = 1000 / APP_CONFIG.rates.control_hz as u64;

    loop {
        let cmd = { *motor_command.lock().await };
        motors.write(cmd);
        Timer::after(Duration::from_millis(period_ms)).await;
    }
}
