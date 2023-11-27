use clap::Parser;

/// The main program of the robot
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The url of the broker <mqtt://server:port> 
    #[arg(short, long)]
    pub mqtt_server: String, 
}
