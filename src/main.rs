pub mod cli;
pub mod mqtt;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use clap::Parser;
use cli::{ressources::Args, system::print_arg};
use mqtt::*;
use std::time::Duration;

const FREQUENCY: f64 = 200.0;

#[allow(dead_code)]
fn hello_world() {
    println!("Good to see you !");
}

fn main() {
    dotenvy::dotenv().ok();
    App::new()
        .insert_resource(Args::parse())
        .add_systems(Startup, hello_world)
        .add_systems(Startup, print_arg)
        .add_systems(Startup, connect_every_broker)
        .add_systems(Update, heartbeat)
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / FREQUENCY,
        )))
        .run();
}
