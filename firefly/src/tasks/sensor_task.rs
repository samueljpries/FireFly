use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};

use crate::drivers::battery::{BatteryMonitor, DummyBatteryMonitor};
use crate::drivers::imu::{DummyImu, ImuDriver};
use crate::drivers::rc_input::{DummyRcInput, RcInput};
use crate::types::{BatterySnapshot, PilotCommand, SensorSnapshot};

#[embassy_executor::task]
pub async fn run(
    mut imu: DummyImu,
    mut battery: DummyBatteryMonitor,
    mut rc: DummyRcInput,
    sensor_snapshot: &'static Mutex<CriticalSectionRawMutex, SensorSnapshot>,
    pilot_command: &'static Mutex<CriticalSectionRawMutex, PilotCommand>,
) {
    loop {
        let imu_result = imu.read_sample();
        let rc_result = rc.read();
        let battery_result = battery.read();

        if let Ok(cmd) = rc_result {
            let mut guard = pilot_command.lock().await;
            *guard = cmd;
        }

        {
            let mut guard = sensor_snapshot.lock().await;
            if let Ok(sample) = imu_result {
                guard.imu = sample;
            }
            if let Ok(power) = battery_result {
                guard.battery = BatterySnapshot { voltage_v: power.battery_v };
            }
            guard.rc_link_ok = rc_result.is_ok();
        }

        Timer::after(Duration::from_millis(2)).await;
    }
}