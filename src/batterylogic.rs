use timer;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BatteryStatus {
    Charging,
    Normal,
    Warning,
    Critical,
    Fatal,
}

pub struct BatteryLogic {
    cap: u8, 
    countdown: u64,
    firstfatal : bool,
    status : BatteryStatus,
    timer: timer::TimerObject,
    updatetimer: timer::TimerObject,
}

// bell, waitsec, fatal
impl BatteryLogic {
    pub fn new(timeout_in_secs: u64) -> BatteryLogic {
        BatteryLogic { 
            cap: 100,
            countdown: 999, 
            firstfatal : true, 
            status : BatteryStatus::Normal,
            updatetimer: timer::TimerObject::new(1, None),
            timer: timer::TimerObject::new(timeout_in_secs, Some(1)),
        }
    }

    pub fn check_alarm_interval(&mut self)  {
        if self.updatetimer.triggered() {
            if self.countdown > 0 {
                self.countdown -= 1;
            }
        }

        if self.timer.triggered() {
            self.countdown = self.alarm_rules();
            self.timer = timer::TimerObject::new(self.countdown, Some(1));
            self.updatetimer = timer::TimerObject::new(1, None);
        }
    }

    pub fn get_countdown(& self) -> u64 {
        self.countdown
    }

    pub fn get_status(& self) -> BatteryStatus {
        self.status
    }

    pub fn get_battery_cap(& self) -> u8 {
        self.cap
    }

    fn alarm_rules(&mut self) -> u64 {
        let (ringbell,sec,fatalevent) = self.rules();
    
        if ringbell {
            super::soundutil::play_ding();
        }

        if !fatalevent && !self.firstfatal {
            self.firstfatal = true;
        }

        if fatalevent && self.firstfatal {
            self.firstfatal = false;
            super::soundutil::play_fatal();
        }
        sec
    }

    fn rules(&mut self) -> (bool, u64, bool) {
        let (_stat, cap, ischarging) = super::extractutil::get_bat_info();
        self.cap = cap;

        if ischarging {
            self.status = BatteryStatus::Charging;
             (false, 600, false)
        }
        else 
        {
            match cap {
                0 ... 9 => {
                    self.status = BatteryStatus::Fatal;
                    (true, 2, true)
                }
                10... 14 => {
                    self.status = BatteryStatus::Critical;
                    (true, 10, false)
                }
                14 ... 20 => {
                    self.status = BatteryStatus::Warning;
                    (true, 60, false)
                }
                _ => {
                    self.status = BatteryStatus::Normal;
                    (false, 120, false)
                }
            }
        }
    }
}
