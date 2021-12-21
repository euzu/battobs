mod observer;
mod config;
mod mqtt;

use std::{thread, time};
use std::process::exit;
use std::sync::{Arc, Mutex};
use battery::Result;
use clap::ArgMatches;
use lazy_static::lazy_static;
use crate::config::{Config, read_config};
use crate::mqtt::{send_mqtt};
use crate::observer::BatteryWatch;

lazy_static! {
    static ref CONFIG: Arc<Mutex<Option<Config>>> = Arc::new(Mutex::new(None));
}

fn main() -> Result<()> {

    let args = get_arguments();
    if args.is_present("list") {
        let battery_watch = BatteryWatch::new(|_cfg: &Config, _on: bool|{});
        battery_watch.list_batteries();
        exit(0);
    }

    let default_path = config::get_default_config_path();
    let config_file = args.value_of("config").unwrap_or(default_path.as_str());
    let config = read_config(config_file);
    let mut cfg_guard = CONFIG.lock().unwrap();
    *cfg_guard = Some(config);
    std::mem::drop(cfg_guard);

    let on_battery_event= |cfg: &Config, on: bool| {
        match &cfg.connection.mqtt {
            Some(mc) => send_mqtt(mc, on),
            _=> ()
        }
    };

    match ctrlc::set_handler(|| std::process::exit(0)) {
        _ => ()
    }
    let handle = thread::spawn(move || {
        let mut battery_watch = BatteryWatch::new(on_battery_event);
        let duration = time::Duration::from_secs(60);
        loop {
            let cfg_guard = CONFIG.lock().unwrap();
            if let Some(ref cfg) = *cfg_guard {
                battery_watch.check(&cfg);
            }
            std::mem::drop(cfg_guard);
            thread::sleep(duration);
        }
    });
    handle.join().unwrap();
    Ok(())
}

fn get_arguments<'a>() -> ArgMatches<'a> {
    clap::App::new("charge-range")
        .version("0.1.0")
        .author("euzu")
        .about("battery charge")
        .arg(clap::Arg::with_name("config")
            .short("c")
            .long("config")
            .takes_value(true)
            .help("The config file"))
        .arg(clap::Arg::with_name("list")
            .short("l")
            .long("list")
            .takes_value(true)
            .help("List batteries"))
        .arg(clap::Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .takes_value(false)
            .help("Print  more log!"))
        .get_matches()
}