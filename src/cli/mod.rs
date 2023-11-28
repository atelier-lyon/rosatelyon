use clap::Parser;

/// The main program of the robot
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The url of the broker <mqtt://server:port>
    #[arg(long, env)]
    pub mqtt_server: String,

    /// The minimum limit of the speed
    #[arg(long, env)]
    pub speed_limit_min: u16,

    /// The maximum limit of the speed
    #[arg(long, env)]
    pub speed_limit_max: u16,

    /// The diameter of the wheels
    #[arg(long, env)]
    pub wheel_diameter: u16,

    /// The gaps between every wheels
    #[arg(long, env)]
    pub wheel_gaps: u16,

    /// The value Kp Ki Kd rho theta phi
    #[clap(long, value_parser, num_args = 6, value_delimiter = ' ', env)]
    pub pid_linear: Vec<u8>,

    /// TODO: Add argument
    #[arg(long, env)]
    pub lidar_pose: u16,

    /// TODO: Add argument
    #[arg(long, env)]
    pub lidar_resolution: u16,
}
