use clap::Parser;
use cli::{Cli, Commands};
use owo_colors::OwoColorize;
use tabled::builder::Builder;
use tabled::settings::Style;

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
        }) => {
            let unused_classes = scan::scan(path, json);
            match unused_classes {
                Ok(classes) => {
                    let mut builder = Builder::default();

                    builder.insert_column(0, ["Unused class"]);
                    builder.insert_column(1, ["Path"]);
                    builder.insert_record(0, vec!["Unused class", "Path"]);

                    classes.iter().for_each(|(k, path)| {
                        builder.push_record(vec![k.clone(), path.display().to_string()])
                    });

                    let mut table = builder.build();
                    table.with(Style::rounded());

                    println!("{}", table);
                }
                Err(error) => println!("{}", error.red()),
            }
        }
        None => {}
    }
}
