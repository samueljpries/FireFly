use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::{Duration, Timer};

use crate::{
    config::APP_CONFIG,
    control::{attitude_controller::AttitudeController, mixer::QuadXMix, rate_controller::RateController},
    estimator::attitude::ComplementaryAttitudeEstimator,
    flight::{arming, commander::FlightCommander},
    types::{MotorCommand, PilotCommand, SensorSnapshot, VehicleState},
};

#[embassy_executor::task]
pub async fn run(
    mut estimator: ComplementaryAttitudeEstimator,
    mut commander: FlightCommander,
    mut attitude: AttitudeController,
    mut rate: RateController,
    mixer: QuadXMix,
    sensor_snapshot: &'static Mutex<CriticalSectionRawMutex, SensorSnapshot>,
    vehicle_state: &'static Mutex<CriticalSectionRawMutex, VehicleState>,
    pilot_command: &'static Mutex<CriticalSectionRawMutex, PilotCommand>,
    motor_command: &'static Mutex<CriticalSectionRawMutex, MotorCommand>,
) {
    let period_ms = 1000 / APP_CONFIG.rates.control_hz as u64;
    let dt = 1.0 / APP_CONFIG.rates.control_hz as f32;

    loop {
        let sensors = { *sensor_snapshot.lock().await };
        let pilot = { *pilot_command.lock().await };

        let state = estimator.update(sensors.imu, sensors.battery.voltage_v, dt);
        let attitude_target = commander.update(pilot);
        let rate_target = attitude.update(attitude_target, state, dt);
        let raw_motor = rate.update(rate_target, state, dt);
        let mixed = mixer.apply(raw_motor, arming::armed(pilot));

        {
            let mut lock = vehicle_state.lock().await;
            *lock = state;
        }

        {
            let mut lock = motor_command.lock().await;
            *lock = mixed;
        }

        Timer::after(Duration::from_millis(period_ms)).await;
    }
}
