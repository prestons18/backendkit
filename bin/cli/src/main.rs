use clap::Parser;
use clap_derive::Parser;

use backendkit_config::load_config;
use backendkit_core;

#[derive(Parser)]
#[command(name = "backendkit")]
#[command(about = "BackendKit CLI for managing backend projects", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "backendkit.toml")]
    config: String,
}

fn main() {
    let cli = Cli::parse();

    let config = match load_config(&cli.config) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    println!("Loaded BackendKit project: {} v{}", config.name, config.version);

    backendkit_core::init();

    println!("BackendKit CLI finished.");
}