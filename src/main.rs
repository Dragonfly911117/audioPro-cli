mod api;
mod commands;
mod config;
mod constants;
mod utils;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "audiopro", about = "Control Audio Pro C20 speaker")]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Config file path (default: ./speaker.json or ~/.config/audiopro/speaker.json)
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    /// Show current speaker status
    Status,
    /// Set or adjust volume (0-100, or +/-N for relative)
    Volume { level: String },
    /// Toggle mute
    Mute,
    /// Start playback
    Play,
    /// Pause playback
    Pause,
    /// Stop playback
    Stop,
    /// Next track
    Next,
    /// Previous track
    Prev,
    /// Switch source (wifi, bluetooth, spotify, line-in, optical, airplay, dlna, usb)
    Source { name: String },
    /// Set equalizer preset (name or number 0-24), or list all if none given
    Eq { preset: Option<String> },
    /// Trigger preset (1-10)
    Preset { number: u8 },
    /// Reboot speaker
    Reboot,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let config = match config::load(cli.config) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let client = match api::build_client() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let result = match cli.command {
        Command::Status => commands::status(&client, &config).await,
        Command::Volume { level } => commands::volume(&client, &config, &level).await,
        Command::Mute => commands::mute(&client, &config).await,
        Command::Play => commands::playback(&client, &config, "play").await,
        Command::Pause => commands::playback(&client, &config, "pause").await,
        Command::Stop => commands::playback(&client, &config, "stop").await,
        Command::Next => commands::playback(&client, &config, "next").await,
        Command::Prev => commands::playback(&client, &config, "prev").await,
        Command::Source { name } => commands::source(&client, &config, &name).await,
        Command::Eq { preset } => commands::eq(&client, &config, preset.as_deref()).await,
        Command::Preset { number } => commands::preset(&client, &config, number).await,
        Command::Reboot => commands::reboot(&client, &config).await,
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
