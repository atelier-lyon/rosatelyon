use rumqttc::{Client, Connection, MqttOptions, QoS};

use crate::cli::ressources::Args;
use bevy::prelude::*;
use std::thread;
use std::time::Duration;

#[derive(Component)]
pub struct ClientConnection(Client, Connection);

#[derive(Component)]
pub struct LoggerRessources {
    channel: Channel,
    message: String,
}

static mut QUEUE_MESSAGE: Vec<LoggerRessources> = Vec::new();
// NOTE: Implement sync for the derive Component
// Could be useful to find a better solution
unsafe impl Sync for ClientConnection {}

#[allow(dead_code)]
pub enum Channel {
    Main,
    Camera,
    Encoder,
    Lidar,
    Debug,
}

fn get_channel(command: &Channel) -> String {
    match command {
        Channel::Main => "main".to_string(),
        Channel::Camera => "camera".to_string(),
        Channel::Encoder => "encoder".to_string(),
        Channel::Lidar => "lidar".to_string(),
        Channel::Debug => "debug".to_string(),
    }
}

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

/// Create a message on queue for mqtt
pub fn log_message(channel: Channel, message: String) {
    unsafe {
        QUEUE_MESSAGE.push(LoggerRessources { channel, message });
    }
}

pub fn send_mqtt_message(
    mut query: Query<&mut ClientConnection>,
    // mut commands: Commands,
) {
    unsafe {
        for mut item in query.iter_mut() {
            for current_log in QUEUE_MESSAGE.iter() {
                let qos = QoS::AtLeastOnce;
                let channel_name = format!("{}/{}", "hello", get_channel(&current_log.channel));
                item.0
                    .publish(channel_name, qos, true, current_log.message.clone())
                    .unwrap();
                thread::sleep(Duration::from_millis(100));
                // TODO: Despawn
                item.1.iter().next();
            }
        }
        QUEUE_MESSAGE.clear();
    }
}
