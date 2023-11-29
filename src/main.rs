pub mod cli;
pub mod mqtt;

use bevy::prelude::*;
use clap::Parser;
use cli::{ressources::Args, system::print_arg};

fn hello_world() {
    println!("Good to see you !");
}

fn main() {
    dotenvy::dotenv().ok();
    App::new()
        .insert_resource(Args::parse())
        .add_systems(Startup, hello_world)
        .add_systems(Startup, print_arg)
        .run();
}
