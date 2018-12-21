use data_yaml;
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
    configuration: data_yaml::Conf,
    firstfatal : bool,
    status : BatteryStatus,
    timer: timer::TimerObject,
    updatetimer: timer::TimerObject,
}

// bell, waitsec, fatal
impl BatteryLogic {
    pub fn new(timeout_in_secs: u64) -> BatteryLogic {
        let c = data_yaml::Conf::new();

        BatteryLogic { 
            cap: 100,
            countdown: 999, 
            configuration: c,
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
             (false, self.configuration.charging_wait, false)
        }
        else 
        {
            match cap {
                _ if cap as u64 <= self.configuration.fatal_limit  => {
                    self.status = BatteryStatus::Fatal;
                    (true, self.configuration.fatal_wait, true)
                }
                _ if cap as u64 <= self.configuration.critical_limit => {
                    self.status = BatteryStatus::Critical;
                    (true, self.configuration.critical_wait, false)
                }
                _ if cap as u64 <= self.configuration.warning_limit => {
                    self.status = BatteryStatus::Warning;
                    (true, self.configuration.warning_wait, false)
                }
                _ => {
                    self.status = BatteryStatus::Normal;
                    (false, self.configuration.normal_wait, false)
                }
            }
        }
    }
}
