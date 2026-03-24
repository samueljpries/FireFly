use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::{Duration, Timer};

use crate::{
    config::APP_CONFIG,
    safety::health::HealthMonitor,
    types::{HealthState, MotorCommand, SensorSnapshot, VehicleState},
};

#[embassy_executor::task]
pub async fn run(
    health: HealthMonitor,
    sensor_snapshot: &'static Mutex<CriticalSectionRawMutex, SensorSnapshot>,
    vehicle_state: &'static Mutex<CriticalSectionRawMutex, VehicleState>,
    motor_command: &'static Mutex<CriticalSectionRawMutex, MotorCommand>,
) {
    let period_ms = 1000 / APP_CONFIG.rates.safety_hz as u64;

    loop {
        let sensors = { *sensor_snapshot.lock().await };
        let _state = { *vehicle_state.lock().await };

        if matches!(health.evaluate(sensors), HealthState::Fault) {
            let mut lock = motor_command.lock().await;
            *lock = MotorCommand::default();
        }

        Timer::after(Duration::from_millis(period_ms)).await;
    }
}
