use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub(crate) debug: u8,

    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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
