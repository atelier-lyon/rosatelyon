use clap::Parser;

/// The main program of the robot
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The url of the broker <server:port>
    #[arg(long, env, value_parser, num_args= 1.., value_delimiter = ' ')]
    pub mqtt_server: Vec<String>,

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
    #[clap(long, env, value_parser, num_args = 6, value_delimiter = ' ')]
    pub pid_linear: Vec<f32>,

    /// The frequency of the encoder
    #[arg(long, env)]
    pub encoder_frequency: u16,

    /// The usb device for the communicaton with the robot
    #[arg(long, env)]
    pub usb_descriptor: String,

    /// The period of the encoder
    #[arg(long, env)]
    pub encoder_period: u16,

    /// The resolution of the lidar
    #[arg(long, env)]
    pub lidar_resolution: u16,
}
