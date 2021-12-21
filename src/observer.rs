use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::io::ErrorKind;
use battery::{Battery, Manager};
use crate::config::Config;

type EventCallback = fn(&Config, bool);

struct NoBatteryError;
impl Error for NoBatteryError {}

impl Debug for NoBatteryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "No battery selected")
    }
}

impl fmt::Display for NoBatteryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No battery selected")
    }
}

pub(crate) struct BatteryWatch {
    manager: Manager,
    battery: Option<Battery>,
    on_event: EventCallback,
    power_on: bool
}

unsafe impl Send for BatteryWatch{}
unsafe impl Sync for BatteryWatch{}


impl BatteryWatch {
    pub(crate) fn new(on_event: EventCallback) -> Self {
        Self {
            manager: Manager::new().unwrap(),
            battery: None,
            on_event,
            power_on: false
        }
    }
    pub fn list_batteries(&self) -> bool {
        match self.manager.batteries() {
            Ok(batteries) => {
                for (idx, maybe_battery) in batteries.enumerate() {
                    match maybe_battery {
                        Ok(battery) => {
                            println!("Battery #{}:", idx);
                            println!("Battery #{:?}:", battery.serial_number());
                            println!("Battery #{:?}:", battery.state_of_charge().value);
                            println!("Battery: {:?}", battery);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        true
    }

    pub fn get_battery_level(&self) -> Result<f32, std::io::Error> {
        match &self.battery {
            Some(akku) => Ok(akku.state_of_charge().value),
            _ => Err(std::io::Error::new(ErrorKind::NotFound, &NoBatteryError {})),
        }
    }

    pub fn check(&mut self, cfg: &Config) -> () {
        if self.battery.is_none() {
           self.battery = Some(self.manager.batteries().unwrap().next().unwrap().unwrap());
        }
        if self.battery.is_none() {
            panic!("No battery found")
        }
        let percent = (self.get_battery_level().unwrap() * 100.0).round() as u8;
        if percent <= cfg.range.min {
            if !self.power_on {
                (self.on_event)(cfg, true);
                self.power_on = true;
            }
        } else if percent >= cfg.range.max {
            if self.power_on {
                (self.on_event)(cfg, false);
                self.power_on = false;
            }
        }
    }
}

/*
vendor: &self.vendor())
model: &self.model())
serial_number: &self.serial_number())
technology: &self.technology())
// common information
state: &self.state())
capacity: &self.state_of_health())
temperature: &self.temperature())
percentage: &self.state_of_charge())
cycle_count: &self.cycle_count())
// energy stats
energy", &self.energy())
energy_full: &self.energy_full())
energy_full_design: &self.energy_full_design())
energy_rate: &self.energy_rate())
voltage: &self.voltage())
// charge stats
time_to_full: &self.time_to_full())
time_to_empty: &self.time_to_empty())
 */
