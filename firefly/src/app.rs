use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use static_cell::StaticCell;

use crate::{
    board::Board,
    config::APP_CONFIG,
    control::{
        attitude_controller::AttitudeController,
        mixer::QuadXMix,
        rate_controller::RateController,
    },
    drivers::{
        battery::DummyBatteryMonitor,
        imu::DummyImu,
        motor_output::DummyMotorOutput,
        rc_input::DummyRcInput,
    },
    estimator::attitude::ComplementaryAttitudeEstimator,
    flight::commander::FlightCommander,
    safety::health::HealthMonitor,
    tasks,
    types::{MotorCommand, PilotCommand, SensorSnapshot, VehicleState},
};

pub type Shared<T> = Mutex<CriticalSectionRawMutex, T>;

pub static SENSOR_SNAPSHOT: StaticCell<Shared<SensorSnapshot>> = StaticCell::new();
pub static VEHICLE_STATE: StaticCell<Shared<VehicleState>> = StaticCell::new();
pub static PILOT_COMMAND: StaticCell<Shared<PilotCommand>> = StaticCell::new();
pub static MOTOR_COMMAND: StaticCell<Shared<MotorCommand>> = StaticCell::new();

pub async fn run(spawner: Spawner) {
    let _board = Board::init();

    let imu = DummyImu;
    let battery = DummyBatteryMonitor;
    let rc = DummyRcInput;
    let motors = DummyMotorOutput;

    let estimator = ComplementaryAttitudeEstimator::new(0.98);
    let rate = RateController::default();
    let attitude = AttitudeController::default();
    let commander = FlightCommander::new();
    let mixer = QuadXMix::new(APP_CONFIG.motors.min_output, APP_CONFIG.motors.max_output);
    let health = HealthMonitor::new(
        APP_CONFIG.battery.low_voltage_v,
        APP_CONFIG.battery.critical_voltage_v,
    );

    let sensor_snapshot = SENSOR_SNAPSHOT.init(Mutex::new(SensorSnapshot::default()));
    let vehicle_state = VEHICLE_STATE.init(Mutex::new(VehicleState::default()));
    let pilot_command = PILOT_COMMAND.init(Mutex::new(PilotCommand::default()));
    let motor_command = MOTOR_COMMAND.init(Mutex::new(MotorCommand::default()));

    spawner
        .spawn(tasks::sensor_task::run(
            imu,
            battery,
            rc,
            sensor_snapshot,
            pilot_command,
        ))
        .ok();

    spawner
        .spawn(tasks::control_task::run(
            estimator,
            commander,
            attitude,
            rate,
            mixer,
            sensor_snapshot,
            vehicle_state,
            pilot_command,
            motor_command,
        ))
        .ok();

    spawner
        .spawn(tasks::motor_task::run(motors, motor_command))
        .ok();

    spawner
        .spawn(tasks::telemetry_task::run(
            vehicle_state,
            pilot_command,
            motor_command,
        ))
        .ok();

    spawner
        .spawn(tasks::safety_task::run(
            health,
            sensor_snapshot,
            vehicle_state,
            motor_command,
        ))
        .ok();

    loop {
        embassy_time::Timer::after_millis(1000).await;
    }
}