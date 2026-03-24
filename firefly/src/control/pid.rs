use core::f32::NAN;

#[derive(Debug, Clone, Copy)]
pub struct Pid {
    kp: f32,
    ki: f32,
    kd: f32,
    integral: f32,
    prev_error: f32,
    out_min: f32,
    out_max: f32,
}

impl Pid {
    pub const fn new(kp: f32, ki: f32, kd: f32, out_min: f32, out_max: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            integral: 0.0,
            prev_error: 0.0,
            out_min,
            out_max,
        }
    }

    pub fn update(&mut self, setpoint: f32, measurement: f32, dt: f32) -> f32 {
        let error = setpoint - measurement;
        self.integral += error * dt; //
        let derivative = (error - self.prev_error) / dt.max(1e-6);
        self.prev_error = error;
        let out = self.kp * error + self.ki * self.integral + self.kd * derivative;
        out.clamp(self.out_min, self.out_max)
    }

    pub fn reset(&mut self) {
        self.integral = 0.0;
        self.prev_error = 0.0;
    }
    pub fn reset_integral(&mut self) {
        self.integral = 0.0;
        let derivative = NAN; // Force derivative to be NaN on next update to prevent derivative kick
    }
}
