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
#[derive(Debug)]
pub enum Channel {
    Main,
    Camera,
    Encoder,
    Lidar,
    Debug,
}

impl Channel {
    // Create a iter for the function connect client
    pub fn into_iter() -> core::array::IntoIter<Channel, 5> {
        [
            Channel::Main,
            Channel::Camera,
            Channel::Encoder,
            Channel::Lidar,
            Channel::Debug,
        ]
        .into_iter()
    }
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
        let (client, connection) = connect_client(
            args.mqtt_client_name.clone(),
            broker,
            args.mqtt_main_topic.clone(),
        );
        commands.spawn(ClientConnection(client, connection));
    }
}

/// Connect a client with a mqtt broker
/// Format of the broker <server:port>
///
/// Return: ClientConnection
fn connect_client(client_name: String, broker: &str, main_topic: String) -> (Client, Connection) {
    let args = broker.split(':').collect::<Vec<&str>>();
    println!("Connecting to {} with {} port", args[0], args[1]);
    let mqtt_options = MqttOptions::new(client_name, args[0], args[1].parse::<u16>().unwrap());
    let (mut client, connection) = Client::new(mqtt_options, 10);
    for channel in Channel::into_iter() {
        let channel_name = format!("{}/{}", main_topic, get_channel(&channel));
        client.subscribe(channel_name, QoS::AtMostOnce).unwrap();
    }
    (client, connection)
}

/// Add a message on the log queue
pub fn log_message(channel: Channel, message: String) {
    unsafe {
        QUEUE_MESSAGE.push(LoggerRessources { channel, message });
    }
}

/// Flush the queue of log
/// TODO: Multi-thread
pub fn send_mqtt_message(mut query: Query<&mut ClientConnection>, args: Res<Args>) {
    unsafe {
        for mut item in query.iter_mut() {
            for current_log in QUEUE_MESSAGE.iter() {
                let qos = QoS::AtLeastOnce;
                let channel_name = format!(
                    "{}/{}",
                    args.mqtt_main_topic.clone(),
                    get_channel(&current_log.channel)
                );
                item.0
                    .publish(channel_name, qos, true, current_log.message.clone())
                    .unwrap();
                thread::sleep(Duration::from_millis(100));
                item.1.iter().next();
            }
        }
        QUEUE_MESSAGE.clear();
    }
}
