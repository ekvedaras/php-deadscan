use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Scan for unused PHP files")]
    Scan {
        path: PathBuf,
        #[arg(short, long, help = "Ignore these paths")]
        ignore: Vec<PathBuf>,
        #[arg(short, long, help = "Entry points")]
        entry: Vec<PathBuf>,
        #[arg(short, long, help = "Only output unused files")]
        only_unused: bool,
        #[arg(short, long, help = "Fail on unused files")]
        fail_on_unused: bool,
        #[arg(short, long, help = "Output as JSON")]
        json: bool,
    },
}

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
        }) => {
            println!("Scanning: {:?}, --json?={:?}", path.display(), json);
        }
        None => {}
    }
}
