#![allow(dead_code)]

use crate::helpers;
use tauri::App;

// Invokes the Audio Resource Manager for Audio Lion
pub fn init(app: &mut App) {
    match helpers::configuration::read_config_file(app.handle()) {
        Ok(_config) => {}
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

/// # Commands
/// - Play
/// - Pause
/// - Resume
/// - Stop
pub enum AudioCommands {
    Play,
    Pause,
    Resume,
    Skip,
    Stop,
}

pub fn handle_audio_command(command: AudioCommands) {
    match command {
        AudioCommands::Play => {}
        AudioCommands::Pause => {}
        AudioCommands::Resume => {}
        AudioCommands::Skip => {}
        AudioCommands::Stop => {}
    }
}