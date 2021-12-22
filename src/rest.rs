use crate::config::RestConfig;
use reqwest;
use strfmt::strfmt;

macro_rules! map {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub(crate) fn send_rest(cfg: &RestConfig, on: bool) -> () {
    let payload: &String;
    if on {
        payload = &cfg.payload.on;
    } else {
        payload = &cfg.payload.off;
    }

    match strfmt(&cfg.url, &map!["payload".to_string() => payload]) {
        Ok(url) => {
            match reqwest::blocking::get(url) {
                Ok(res) => {
                    if !res.status().is_success() {
                        println!("Request failed {}", res.status());
                    }
                }
                Err(e) => println!("Request failed {}", e),
            }
        }
        Err(e) => println!("Misformed url {}", e),
    }
    ()
}
