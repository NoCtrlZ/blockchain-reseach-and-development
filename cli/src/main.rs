extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
    let app = App::new("sexydynamite")
        .version("0.1.0")
        .author("NoCtrlZ <phantomofrotten@gmail.com>")
        .about("Test Blockchain Server")
        .subcommand(SubCommand::with_name("init")
            .about("init project")
        );
    let matches = app.get_matches();

    if let Some(ref matches) = matches.subcommand_matches("init") {
        println!("init wallet");
    }
}
