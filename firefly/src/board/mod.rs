pub mod clocks;
pub mod i2c_bus;
pub mod pins;
pub mod pwm;
pub mod spi_bus;
pub mod uart;

pub struct Board;

impl Board {
    pub fn init() -> Self {
        // Replace with actual esp-hal peripheral initialization.
        Self
    }
}
