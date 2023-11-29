use rumqttc::{Client, MqttOptions, QoS};
use std::time::Duration;

pub fn connect_client(broker: &String) {
    let mut mqttoptions = MqttOptions::new("rumqtt-sync", broker.clone(), 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (mut client, _connection) = Client::new(mqttoptions, 10);
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).unwrap();
    let payload = format!("publish {}", broker);
    client
        .publish("hello/rumqtt", QoS::AtLeastOnce, false, payload)
        .unwrap();
}
