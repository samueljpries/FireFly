#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FlightMode {
    #[default]
    Disarmed,
    Stabilize,
    Acro,
    AltitudeHold,
    Land,
    Failsafe,
}
