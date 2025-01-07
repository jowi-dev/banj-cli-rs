use clap::{Parser, Subcommand};
use std::process::Command;


#[derive(Parser)]
#[command(name = "banj")]
#[command(about = "A CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Development commands
    Develop {
        /// Command to run
        #[arg(short, long, default_value="fish")]
        command: String,
    },
    // You can add more top-level commands here
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Develop { command } => {
            Command::new("nix")
                .arg("develop")
                .arg("--command")
                .arg(command)
                .status()
                .expect("Failed to execute command");
        }
    }
}
