use crate::config::Config;
use battery::{Battery, Manager, State};
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Formatter};

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
    on_event: EventCallback,
}

unsafe impl Send for BatteryWatch {}
unsafe impl Sync for BatteryWatch {}

impl BatteryWatch {
    pub(crate) fn new(on_event: EventCallback) -> Self {
        Self {
            manager: Manager::new().unwrap(),
            on_event,
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

    fn get_battery_charging(&self, battery: &Battery) -> bool {
        match battery.state() {
            State::Unknown => return true,
            State::Charging => return true,
            State::Full => return true,
            _ => return false,
        }
    }

    fn get_battery_level(&self, battery: &Battery) -> f32 {
        return battery.state_of_charge().value;
    }

    pub fn check(&mut self, cfg: &Config) -> () {
        match self.manager.batteries().unwrap().next().unwrap() {
            Ok(battery) => {
                let is_charging = self.get_battery_charging(&battery);
                let percent = (self.get_battery_level(&battery) * 100.0).round() as u8;
                if percent <= cfg.range.min {
                    if !is_charging {
                        (self.on_event)(cfg, true);
                    }
                } else if percent >= cfg.range.max {
                    if is_charging {
                        (self.on_event)(cfg, false);
                    }
                }
            }
            _ => println!("No battery found"),
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
