use crate::batterylogic;
use crate::timer;
use crate::colordefs;

pub struct Blinker {
    state_is_on: bool,
    timer: timer::TimerObject,
}

// bell, waitsec, fatal
impl Blinker {
    pub fn new(timeout_requested: u64) -> Blinker {
        Blinker { 
            state_is_on: false,
            timer: timer::TimerObject::new(timeout_requested, None),
        }
    }

    pub fn get_blink_color(&mut self, status : batterylogic::BatteryStatus) -> [f32;4] {
        // flip the state 
        if self.timer.triggered() {
            self.state_is_on = ! self.state_is_on;
        }

        if self.state_is_on {
            match status {
                batterylogic::BatteryStatus::Warning => colordefs::COLOR_YELLOW,
                batterylogic::BatteryStatus::Critical => colordefs::COLOR_ORANGE,
                batterylogic::BatteryStatus::Fatal => colordefs::COLOR_RED,
                _ => colordefs::COLOR_GRAY2,
            }
        } 
        else {
            colordefs::COLOR_GRAY2
        }
    }
}