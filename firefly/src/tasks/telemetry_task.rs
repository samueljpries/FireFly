use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::{Duration, Timer};

use crate::{config::APP_CONFIG, types::{MotorCommand, PilotCommand, VehicleState}};

#[embassy_executor::task]
pub async fn run(
    vehicle_state: &'static Mutex<CriticalSectionRawMutex, VehicleState>,
    pilot_command: &'static Mutex<CriticalSectionRawMutex, PilotCommand>,
    motor_command: &'static Mutex<CriticalSectionRawMutex, MotorCommand>,
) {
    let period_ms = 1000 / APP_CONFIG.rates.telemetry_hz as u64;

    loop {
        let _state = { *vehicle_state.lock().await };
        let _pilot = { *pilot_command.lock().await };
        let _motors = { *motor_command.lock().await };

        // Replace with UART/radio/MAVLink publishing.
        Timer::after(Duration::from_millis(period_ms)).await;
    }
}
