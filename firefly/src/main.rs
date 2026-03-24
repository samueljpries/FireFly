#![no_std]
#![no_main]

mod app;
mod board;
mod comms;
mod config;
mod control;
mod drivers;
mod estimator;
mod flight;
mod safety;
mod tasks;
mod types;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[esp_hal_embassy::main]
async fn main(spawner: embassy_executor::Spawner) {
    app::run(spawner).await;
}
