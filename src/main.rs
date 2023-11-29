pub mod cli;
pub mod mqtt;

use bevy::prelude::*;
use clap::Parser;
use cli::Args;

use crate::mqtt::connect_client;

fn hello_world() {
    println!("Hello World");
}

fn main() {
    dotenvy::dotenv().ok();
    let args = Args::parse();
    println!("{:#?}", args);
    connect_client(args.mqtt_server.get(0).unwrap());
    App::new().add_systems(Startup, hello_world).run();
}
