use anyhow::Result;

/// Linux implementation: uses RPPAL to drive GPIO pins.
#[cfg(target_os = "linux")]
mod imp {
    use super::Result;
    use rppal::gpio::{Gpio, OutputPin};

    /// Controls two DC motors via L293D inputs (EN pins tied high).
    pub struct MotorController {
        in1: OutputPin,
        in2: OutputPin,
        in3: OutputPin,
        in4: OutputPin,
    }

    impl MotorController {
        /// Initialize GPIO pins for motor control.
        /// Pin pairs: (17,18) for motor A; (27,22) for motor B.
        pub fn new() -> Result<Self> {
            let gpio = Gpio::new()?;
            Ok(Self {
                in1: gpio.get(17)?.into_output(),
                in2: gpio.get(18)?.into_output(),
                in3: gpio.get(27)?.into_output(),
                in4: gpio.get(22)?.into_output(),
            })
        }

        /// Drive both motors forward
        pub fn forward(&mut self) {
            self.in1.set_high();
            self.in2.set_low();
            self.in3.set_high();
            self.in4.set_low();
        }

        /// Drive both motors backward
        pub fn backward(&mut self) {
            self.in1.set_low();
            self.in2.set_high();
            self.in3.set_low();
            self.in4.set_high();
        }

        /// Turn left: left motor backward, right motor forward
        pub fn turn_left(&mut self) {
            self.in1.set_low();
            self.in2.set_high();
            self.in3.set_high();
            self.in4.set_low();
        }

        /// Turn right: left motor forward, right motor backward
        pub fn turn_right(&mut self) {
            self.in1.set_high();
            self.in2.set_low();
            self.in3.set_low();
            self.in4.set_high();
        }

        /// Stop all motors
        pub fn stop(&mut self) {
            self.in1.set_low();
            self.in2.set_low();
            self.in3.set_low();
            self.in4.set_low();
        }
    }

    pub use MotorController;
}

/// Stub implementation for non-Linux platforms (no-op).
#[cfg(not(target_os = "linux"))]
pub struct MotorController;

#[cfg(not(target_os = "linux"))]
impl MotorController {
    pub fn new() -> Result<Self> { Ok(Self) }
    pub fn forward(&mut self) {}
    pub fn backward(&mut self) {}
    pub fn turn_left(&mut self) {}
    pub fn turn_right(&mut self) {}
    pub fn stop(&mut self) {}
}