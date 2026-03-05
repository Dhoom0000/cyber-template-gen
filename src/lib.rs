use std::process::exit;

use clap::{Parser, Subcommand};
use colorful::Colorful;

use crate::files::parse_toml;

mod files;

#[derive(Parser)]
#[command(version = "0.1", long_about = None)]
#[command(about = "Create and maintain a cyber security template.")]
#[command(arg_required_else_help = true, subcommand_required = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new template directory
    New {
        /// Folder name
        #[arg(long, default_value_t = String::from("cyber-template"))]
        name: String,
    },
}

pub fn parse_args() {
    match Args::try_parse() {
        Ok(args) => match args.command {
            Commands::New { name } => {
                let info = "Creating".blue().bold();
                println!("{info} `{name}`");
                parse_toml(None, name);
            }
        },

        Err(err) => {
            if err.use_stderr() {
                err.print().unwrap();
            } else {
                println!("{}", err.render().ansi());
                exit(1);
            }
            let msg_1 = "cyber-template-gen ".blue().bold();
            let msg_2 = "new".yellow();

            println!("\nUse {msg_1}{msg_2} to create a template in working directory.");

            exit(0);
        }
    }
}
