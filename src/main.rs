
pub mod cli;

use bevy::prelude::*;
use clap::Parser;
use cli::Args;

fn hello_world() {
    println!("Hello World");
}

fn main() {
    dotenvy::dotenv().ok();
    let args = Args::parse();
    println!("Hello {}!", args.mqtt_server);
    App::new().add_systems(Startup, hello_world).run();
}
