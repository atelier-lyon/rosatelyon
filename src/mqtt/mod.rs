use rumqttc::{Client, Connection, MqttOptions, QoS};
use std::thread;
use std::time::Duration;

use crate::cli::ressources::Args;
use bevy::prelude::*;

#[derive(Component)]
struct ClientConnection(Connection);

// NOTE: Implement sync for the derive Component
// Could be useful to find a better solution
unsafe impl Sync for ClientConnection {}

/// Connect every the program to every broker
/// Add a Component, ClientConnection for everty new connection
pub fn connect_every_broker(mut commands: Commands, args: Res<Args>) {
    for broker in args.mqtt_server.iter() {
        commands.spawn(connect_client(args.mqtt_client_name.clone(), broker));
    }
}

/// Connect a client with a mqtt broker
/// Format of the broker <server:port>
///
/// Return: ClientConnection
fn connect_client(client_name: String, broker: &str) -> ClientConnection {
    let args = broker.split(':').collect::<Vec<&str>>();
    println!("Connecting to {} with {} port", args[0], args[1]);
    let mqtt_options = MqttOptions::new(client_name, args[0], args[1].parse::<u16>().unwrap());
    let (client, mut connection) = Client::new(mqtt_options, 10);
    thread::spawn(move || publish(client));
    for _ in connection.iter().enumerate() {}
    ClientConnection(connection)
}

fn publish(mut client: Client) {
    client.subscribe("hello/world", QoS::AtMostOnce).unwrap();
    let payload = "coucou";
    let qos = QoS::AtLeastOnce;

    client.publish("hello/world", qos, true, payload).unwrap();

    thread::sleep(Duration::from_secs(1));
}
