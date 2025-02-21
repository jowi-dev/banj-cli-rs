use clap::{Parser, Subcommand};
use strum_macros::Display;
use std::process::Command as ProcessCommand;
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
    Clean {
        #[arg(short, long, default_value_t=false)]
        all: bool,
    },
    Develop {
        #[arg(short, long, default_value="fish")]
        command: String,
    },
    Rebuild,
    UserSwitch,
    Sleep,
    Monitor,
    Display,
    Project {
        #[command(subcommand)]
        command: ProjectCommands
    },
    Log {
        #[command(subcommand)]
        command: LogCommands

    },
    #[command(external_subcommand)]
    AI(Vec<String>)        // You can add more top-level commands here
}


#[derive(Subcommand)]
enum ProjectCommands {
    // Initializes a Project of the given template type in the current directory
    Init{
        // Specifies the project type to initialize
        #[command(subcommand)]
        template: ProjectTemplates
    },
}

#[derive(Subcommand)]
enum LogCommands{
    List,
    New {
        input: String
    },
    Publish,
    Edit,
    Show, 
    Delete
}

#[derive(Subcommand, Eq, PartialEq, Display)]
enum ProjectTemplates {
    //Elixir Script is an escript ready project
    #[strum(serialize="elixir-script")]
    ElixirScript,
    #[strum(serialize="odin")]
    Odin,
    #[strum(serialize="bash")]
    Bash,
    //Elixir Phoenix as the name implies initializes a project with a database, elixir, phoenix
    //ready to run
    #[strum(serialize="elixir-phoenix")]
    ElixirPhoenix,
    //Rust is currently the community default rust flake
    Rust,
    // TODO
    #[strum(serialize="ocaml")]
    OCaml
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Develop { command } => {
            ProcessCommand::new("nix")
                .arg("develop")
                .arg("--command")
                .arg(command)
                .status()
                .expect("Failed to execute command");
        }
        Commands::Rebuild => rebuild(),
        Commands::UserSwitch => git_switch_user(),
        Commands::Sleep => sleep(),
        Commands::Clean { all } => clean(*all),
        Commands::Monitor => todo!(),
        Commands::Display => todo!(),
        Commands::Project {command} => project(&command),
        Commands::Log {command} => log(&command),
        Commands::AI(args) => ai(&args)
    }
}

fn rebuild() {
    let config = env::var("CONFIG_DIR").expect("config dir not set");
    let flake = env::var("FLAKE").expect("unknown build for configuration");
    if cfg!(target_os = "macos"){
        ProcessCommand::new("darwin-rebuild")
            .arg("switch")
            .arg("--flake")
            .arg(config + "/.#" + &flake)
            .status()
            .expect("Failed to execute command");
    } else if config == "HMCONFIG" {
        ProcessCommand::new("home-manager")
            .arg("switch")
            .arg("--flake")
            .arg(config + "/.#" + &flake)
            .status()
            .expect("Failed to execute command");
    } else if cfg!(target_os = "linux"){
        ProcessCommand::new("sudo")
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

fn git_switch_user() {
    ProcessCommand::new("gh")
        .arg("auth")
        .arg("switch")
        .status()
        .expect("Failed to switch user");

    rebuild();
}

fn sleep() {
    if cfg!(target_os = "macos") {
        ProcessCommand::new("pmset")
            .arg("sleepnow")
            .status()
            .expect("Failed to sleep");

    } else if cfg!(target_os = "linux"){
        ProcessCommand::new("systemctl")
            .arg("hibernate")
            .status()
            .expect("Failed to sleep");
    }
}

fn clean(clean_all : bool) {
    if clean_all {
        ProcessCommand::new("sudo")
            .arg("nix-collect-garbage")
            .arg("-d")
            .status()
            .expect("");
        ProcessCommand::new("sudo")
            .arg("nix-store")
            .arg("--gc")
            .status()
            .expect("");
        ProcessCommand::new("sudo")
            .arg("nix-store")
            .arg("--optimise")
            .status()
            .expect("");
    }
    else{
        ProcessCommand::new("sudo")
            .arg("nix-collect-garbage")
            .arg("-d")
            .status()
            .expect("");
    }
}

fn project(command : &ProjectCommands) {
    match command {
        ProjectCommands::Init { template } if *template == ProjectTemplates::Rust => {
            ProcessCommand::new("nix")
                .arg("flake")
                .arg("init")
                .arg("-t")
                .arg("templates#rust")
                .status()
                .expect("Failed to initialize project");

        },
        ProjectCommands::Init {template} => {
            let config = env::var("CONFIG_DIR").expect("config dir not set");
            ProcessCommand::new("nix")
                .arg("flake")
                .arg("init")
                .arg("-t")
                .arg(config + "#" + &template.to_string())
                .status()
                .expect("Failed to initialize project");
        },
    }
}

fn log(command: &LogCommands) {
    let log_dir = env::var("LOG_DIR_PERSONAL").expect("BanjOS: Failed to find log directory");

    match command {
        LogCommands::New { input } => {
            ProcessCommand::new("touch")
                .arg(log_dir + "/" + input)
                .status()
                .expect("Failed to create new log file");
        },
        LogCommands::List =>{
            //lsd -1 --group-directories-first -R -I result ~/log-dir
            ProcessCommand::new("lsd")
                .arg(log_dir)
                .arg("-1")
                .arg("--group-directories-first")
                .arg("-R")
                .status()
                .expect("Failed to list logs");
        },
        LogCommands::Edit => todo!(),
        LogCommands::Show => todo!(),
        LogCommands::Delete => todo!(),
        LogCommands::Publish => todo!()
    }
}

fn ai(args: &Vec<String>) {
    if args.len() <= 1 {  // Only has "ai" or is empty
        ProcessCommand::new("fnord")
            .arg("--help")
            .status()
            .expect("failed to be helpful");
    } else {
        ProcessCommand::new("fnord")
            .args(&args[1..])  // Skip the "ai" argument
            .status()
            .expect("failed to do ai thing");
    }
}
