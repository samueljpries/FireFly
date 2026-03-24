use crate::flight::modes::FlightMode;

pub fn choose_failsafe(radio_ok: bool, battery_critical: bool, imu_ok: bool) -> FlightMode {
    if !imu_ok {
        FlightMode::Failsafe
    } else if !radio_ok || battery_critical {
        FlightMode::Land
    } else {
        FlightMode::Stabilize
    }
}
