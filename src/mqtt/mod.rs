use rumqttc::{Client, Connection, MqttOptions, QoS};

use crate::cli::ressources::Args;
use bevy::prelude::*;
use std::thread;
use std::time::Duration;

#[derive(Component)]
pub struct ClientConnection(Client, Connection);

// NOTE: Implement sync for the derive Component
// Could be useful to find a better solution
unsafe impl Sync for ClientConnection {}

/// Connect every the program to every broker
/// Add a Component, ClientConnection for everty new connection
pub fn connect_every_broker(mut commands: Commands, args: Res<Args>) {
    for broker in args.mqtt_server.iter() {
        let (client, connection) = connect_client(args.mqtt_client_name.clone(), broker);
        commands.spawn(ClientConnection(client, connection));
    }
}

/// Connect a client with a mqtt broker
/// Format of the broker <server:port>
///
/// Return: ClientConnection
fn connect_client(client_name: String, broker: &str) -> (Client, Connection) {
    let args = broker.split(':').collect::<Vec<&str>>();
    println!("Connecting to {} with {} port", args[0], args[1]);
    let mqtt_options = MqttOptions::new(client_name, args[0], args[1].parse::<u16>().unwrap());
    let (mut client, connection) = Client::new(mqtt_options, 10);
    client.subscribe("hello/world", QoS::AtMostOnce).unwrap();
    (client, connection)
}

pub fn heartbeat(mut query: Query<&mut ClientConnection>) {
    for mut item in query.iter_mut() {
        println!("heartbeat !");
        let payload = "coucou";
        let qos = QoS::AtLeastOnce;

        item.0.publish("hello/world", qos, true, payload).unwrap();
        thread::sleep(Duration::from_millis(100));
        item.1.iter().next();
    }
}
