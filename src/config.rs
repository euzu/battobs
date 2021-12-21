use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Range {
    pub min : u8,
    pub max: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Payload {
   pub off: String,
   pub on: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RestConfig {
    pub url: String,
    pub payload: Payload,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct MqttConfig {
    pub server: String,
    pub port: u16,
    pub channel: String,
    pub payload: Payload
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ConnectionConfig {
    pub rest: Option<RestConfig>,
    pub mqtt: Option<MqttConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub range: Range,
    pub connection: ConnectionConfig,
}

pub(crate) fn get_exe_path() -> std::path::PathBuf {
    let default_path = std::path::PathBuf::from("./");
    let current_exe = std::env::current_exe();
    let path: std::path::PathBuf = match current_exe {
        Ok(exe) => exe.parent().map_or(default_path, |p| p.to_path_buf()),
        Err(_) => default_path
    };
    path
}

pub(crate) fn open_file(file_name: &PathBuf) -> std::fs::File {
    let file = match std::fs::File::open(file_name) {
        Ok(file) => file,
        Err(_) => {
            println!("cant open file: {:?}", file_name);
            std::process::exit(1);
        }
    };
    file
}

pub(crate) fn get_default_config_path() -> String {
    let path: std::path::PathBuf = get_exe_path();
    let config_path = path.join("config.yml");
    String::from(if config_path.exists() {
        config_path.to_str().unwrap_or("./config.yml")
    } else {
        "./config.yml"
    })
}

pub(crate) fn read_config(config_file: &str) -> Config {
    let cfg: Config = match serde_yaml::from_reader(open_file(&std::path::PathBuf::from(config_file))) {
        Ok(c) => c,
        Err(e) => panic!("cant read config file: {}", e)
    };
    cfg
}