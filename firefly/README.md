# Drone Firmware



## Architecture

- `board/`: ESP32-C3 hardware setup and peripheral ownership
- `drivers/`: hardware-facing drivers and traits
- `estimator/`: sensor fusion and state estimation
- `control/`: PID loops, attitude/rate controllers, motor mixing
- `flight/`: arming rules and flight-mode command translation
- `safety/`: health checks and fault reactions
- `comms/`: telemetry, radio, MAVLink, logging
- `tasks/`: Embassy runtime tasks

## Current state

Not production flight code yet

Tasks Completed:
- dummy drivers for IMU, battery, RC input, and motor output
- complementary-filter attitude estimator
- attitude-to-rate and rate-to-motor control flow
- basic safety task that cuts motors on fault

To Do:
- real `esp-hal` peripheral setup
- calibrated IMU driver
- DShot / OneShot / PWM ESC driver
- robust arming state machine
- sensor calibration storage
- persistent configuration
- telemetry protocol implementation
- real failsafe strategy validation
- watchdog reset integration
- formal tests / HIL / SIL

## Bring-up order

1. Replace `DummyImu` with a real IMU driver
2. Replace `DummyMotorOutput` with a real ESC output driver
3. Add RC receiver parsing
4. Validate estimator output on the bench
5. Tune rate loop before attitude loop
6. Add logging and telemetry
7. Add failsafe and watchdog hardening
