use rumqttc::{Client, Connection, MqttOptions};
use std::time::Duration;

use crate::cli::ressources::Args;
use bevy::prelude::*;

#[derive(Component)]
struct ClientConnection(Client, Connection);

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
fn connect_client(client_name: String, broker: &String) -> ClientConnection {
    // TODO: Extract the port from the broker string <server:port>
    let mut mqttoptions = MqttOptions::new(client_name, broker, 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, connection) = Client::new(mqttoptions, 10);
    println!("Connected to {}", broker);
    ClientConnection(client, connection)
}
