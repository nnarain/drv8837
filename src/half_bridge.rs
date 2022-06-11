//
// half_bridge.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jun 09 2022
//

use embedded_hal::{
    digital::v2::OutputPin,
    PwmPin,
};

/**
 * Half-bridge configuration
*/
pub struct HalfBridgeDriver<EnablePin, In1Pin, In2Pin> {
    enable: EnablePin,
    in1: In1Pin,
    in2: In2Pin,
}

impl<EnablePin, In1Pin, In2Pin> HalfBridgeDriver<EnablePin, In1Pin, In2Pin>
    where EnablePin: OutputPin, In1Pin: PwmPin, In2Pin: PwmPin {

    /**
     * Create a new driver
     */
    pub fn new(enable: EnablePin, in1: In1Pin, in2: In2Pin) -> Self {
        HalfBridgeDriver {
            enable,
            in1,
            in2
        }
    }

    /**
     * Enable PWM outputs
     */
    pub fn enable(&mut self) -> Result<(), EnablePin::Error> {
        self.enable.set_high()
    }

    /**
     * Set duty cycle for each motor
     */
    pub fn set_duty(&mut self, duty1: In1Pin::Duty, duty2: In2Pin::Duty) {
        self.in1.set_duty(duty1);
        self.in2.set_duty(duty2);
    }

    /**
     * Release hardware ownership
     */
    pub fn free(self) -> (EnablePin, In1Pin, In2Pin) {
        (self.enable, self.in1, self.in2)
    }
}
