use clap::{Parser, Subcommand};
use std::process::Command;
use std::env;


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
    Rebuild,
    Sleep,
    Clean,
    Monitor,
    Display,
    Project
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
        Commands::Rebuild => rebuild(),
        Commands::Sleep => sleep(),
        Commands::Clean => todo!(),
        Commands::Monitor => todo!(),
        Commands::Display => todo!(),
        Commands::Project => todo!()
    }
}

fn rebuild() {
    let config = env::var("CONFIG_DIR").expect("config dir not set");
    let flake = env::var("FLAKE").expect("unknown build for configuration");
    if cfg!(target_os = "macos"){
        Command::new("darwin-rebuild")
            .arg("switch")
            .arg("--flake")
            .arg(config + "/.#" + &flake)
            .status()
            .expect("Failed to execute command");
    } else if cfg!(target_os = "linux"){
        Command::new("sudo")
            .arg("nixos-rebuild")
            .arg("switch")
            .arg("--flake")
            .arg(config + "/.#" + &flake)
            .status()
            .expect("Failed to execute command");
    } else{
        panic!("System not supported")
    }
}

fn sleep() {
    if cfg!(target_os = "macos") {
        Command::new("pmset")
            .arg("sleepnow")
            .status()
            .expect("Failed to sleep");

    } else if cfg!(target_os = "linux"){
        Command::new("systemctl")
            .arg("hibernate")
            .status()
            .expect("Failed to sleep");
    }
}
