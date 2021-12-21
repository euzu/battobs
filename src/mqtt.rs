use crate::config::MqttConfig;
use rumqttc::Event::Incoming;
use rumqttc::{Client, MqttOptions, Packet, QoS};

pub(crate) fn send_mqtt(cfg: &MqttConfig, on: bool) -> () {
    let payload: &String;
    if on {
        payload = &cfg.payload.on;
    } else {
        payload = &cfg.payload.off;
    }
    let mqttoptions = MqttOptions::new("charge-range", &cfg.server, cfg.port);
    //mqttoptions.set_keep_alive(time::Duration::from_secs(5));
    let (mut client, mut eventloop) = Client::new(mqttoptions, 10);
    let msg = String::from(payload).into_bytes();
    //client.subscribe(&cfg.channel, QoS::ExactlyOnce).unwrap();
    match client.publish(&cfg.channel, QoS::AtLeastOnce, false, msg) {
        Ok(_) => (),
        Err(e) => println!("{}", e.to_string()),
    }

    // Iterate to poll the eventloop for connection progress
    for (_i, notification) in eventloop.iter().enumerate() {
        match notification {
            Ok(ref r) => match r {
                Incoming(i) => match i {
                    Packet::PubAck(_) => {
                        break;
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => break,
        }
        // println!("Notification = {:?}", notification);
    }
    match client.disconnect() {
        _ => (),
    }
    ()
}
