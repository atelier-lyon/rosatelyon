use rumqttc::{Client, MqttOptions};
use std::time::Duration;

use crate::cli::ressources::Args;
use bevy::prelude::*;

pub fn connect_every_broker(args: Res<Args>) {
    for broker in args.mqtt_server.iter() {
        connect_client(broker);
    }
}

fn connect_client(broker: &String) {
    let mut mqttoptions = MqttOptions::new("Robot", broker, 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut _client, mut _connection) = Client::new(mqttoptions, 10);
    println!("Connected to {}", broker);
}
