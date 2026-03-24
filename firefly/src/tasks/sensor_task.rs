use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::{Duration, Timer};

use crate::{
    config::APP_CONFIG,
    drivers::{battery::BatteryMonitor, imu::ImuDriver, rc_input::RcInput},
    types::{PilotCommand, SensorSnapshot},
};

#[embassy_executor::task]
pub async fn run<I, B, R>(
    mut imu: I,
    mut battery: B,
    mut rc: R,
    sensor_snapshot: &'static Mutex<CriticalSectionRawMutex, SensorSnapshot>,
    pilot_command: &'static Mutex<CriticalSectionRawMutex, PilotCommand>,
) where
    I: ImuDriver + 'static,
    B: BatteryMonitor + 'static,
    R: RcInput + 'static,
{
    let period_ms = 1000 / APP_CONFIG.rates.sensor_hz as u64;
    let _ = imu.init();

    loop {
        let imu_sample = imu.read_sample().unwrap_or_default();
        let batt = battery.read();
        let rc_cmd = rc.read();
        let link_ok = rc.link_ok();

        {
            let mut lock = sensor_snapshot.lock().await;
            *lock = SensorSnapshot {
                imu: imu_sample,
                battery: batt,
                rc_link_ok: link_ok,
            };
        }

        {
            let mut lock = pilot_command.lock().await;
            *lock = rc_cmd;
        }

        Timer::after(Duration::from_millis(period_ms)).await;
    }
}
