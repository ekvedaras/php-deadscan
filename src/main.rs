use clap::Parser;
use cli::{Cli, Commands};

mod cli;
mod scan;

fn main() {
    let cli = Cli::parse();

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    match &cli.command {
        Some(Commands::Scan {
            path,
            ignore: _,
            entry: _,
            only_unused: _,
            fail_on_unused: _,
            json,
        }) => scan::scan(path, json),
        None => {}
    }
}
