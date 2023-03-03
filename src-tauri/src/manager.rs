#![allow(dead_code)]

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{audio::player::Player, helpers};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tauri::App;

pub fn init(app: &mut App) {
    match helpers::configuration::read_config_file(app.handle()) {
        Ok(_config) => {}
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AudioCommands {
    Play,
    Pause,
    Resume,
    Skip,
    Stop,
}

struct PlayCommand {
    player: Arc<Mutex<Player>>,
}

impl PlayCommand {
    fn new(player: Arc<Mutex<Player>>) -> Self {
        PlayCommand { player }
    }

    fn on_command(&mut self, app_handle: tauri::AppHandle, path: PathBuf) {
        let mut player = self.player.lock().unwrap();
        if !player.has_ended() {
            player.end_current().unwrap();
        }

        println!("Playing: {:?}", path.display());
        player.play_from_path(app_handle, path).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioCommandResult {
    pub success: bool,
    pub is_paused: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayAudioParams {
    pub file_path: String,
}

pub async fn handle_audio_command(
    app_handle: tauri::AppHandle,
    command: AudioCommands,
    play_params: Option<String>,
) -> Result<AudioCommandResult> {
    let player = Arc::new(Mutex::new(Player::new(String::from("Audio Player"), 1.0, 1.0)));

    match command {
        AudioCommands::Play => {
            if let Some(params) = play_params {
                let mut play_command = PlayCommand::new(player.clone());
                let app = app_handle.clone();
                play_command.on_command(app, PathBuf::from(params));
                Ok(AudioCommandResult {
                    success: true,
                    is_paused: player.lock().unwrap().is_paused(),
                })
            } else {
                Err(anyhow!("No params provided for play command."))
            }
        }
        AudioCommands::Pause => Ok(AudioCommandResult {
            success: true,
            is_paused: player.lock().unwrap().is_paused(),
        }),
        AudioCommands::Resume => Ok(AudioCommandResult {
            success: true,
            is_paused: player.lock().unwrap().is_paused(),
        }),
        AudioCommands::Skip => Ok(AudioCommandResult {
            success: true,
            is_paused: player.lock().unwrap().is_paused(),
        }),
        AudioCommands::Stop => Ok(AudioCommandResult {
            success: true,
            is_paused: player.lock().unwrap().is_paused(),
        }),
    }
}
