use bevy::prelude::*;

use super::ressources::Args;

/// Print every args on the stdout
/// Query: Arg
pub fn print_arg(args: Res<Args>) {
    println!("{:#?}", args);
}
