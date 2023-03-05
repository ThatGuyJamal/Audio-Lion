#![allow(dead_code)]

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{audio::player::Player, helpers};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::App;

pub fn init(app: &mut App) {
    match helpers::configuration::read_config_file(app.handle()) {
        Ok(_config) => {}
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
/// Commands for the audio player to handle.
pub enum AudioCommands {
    Play,
    Pause,
    Resume,
    Stop,
}

struct PlayCommand {
    player: Arc<Mutex<Player>>,
}

impl PlayCommand {
    fn new(player: Arc<Mutex<Player>>) -> Self {
        PlayCommand { player }
    }

    fn run(&mut self, app_handle: tauri::AppHandle, path: PathBuf) -> Result<AudioCommandResult> {
        let mut player = self.player.lock().unwrap();

        println!("playing: {:?}", player);

        let path_clone = path.clone();

        if !player.has_ended() {
            player.end_current().unwrap();
        }

        println!("Playing: {:?}", path_clone.display());
        player.play_from_path(app_handle, path).unwrap();

        Ok(AudioCommandResult {
            command_name: "Play".to_string(),
            success: true,
            is_paused: player.is_paused(),
            path: Some(path_clone.to_str().unwrap().to_string()),
        })
    }
}

struct PauseCommand {
    player: Arc<Mutex<Player>>,
}

impl PauseCommand {
    fn new(player: Arc<Mutex<Player>>) -> Self {
        PauseCommand { player }
    }

    fn run(&mut self) -> Result<AudioCommandResult> {
        let mut player = self.player.lock().unwrap();

        if !player.is_paused() && player.is_playing() {
            player.pause().unwrap();

            println!("Pausing: {:?}", player);

            return Ok(AudioCommandResult {
                command_name: "Pause".to_string(),
                success: true,
                is_paused: player.is_paused(),
                path: None,
            });
        }

        Ok(AudioCommandResult {
            command_name: "Pause".to_string(),
            success: false,
            is_paused: player.is_paused(),
            path: None,
        })
    }
}

struct ResumeCommand {
    player: Arc<Mutex<Player>>,
}

impl ResumeCommand {
    fn new(player: Arc<Mutex<Player>>) -> Self {
        ResumeCommand { player }
    }

    fn run(&mut self) -> Result<AudioCommandResult> {
        let mut player = self.player.lock().unwrap();

        if player.is_paused() {
            player.unpause().unwrap();

            println!("Resuming: {:?}", player);

            return Ok(AudioCommandResult {
                command_name: "Resume".to_string(),
                success: true,
                is_paused: player.is_paused(),
                path: None,
            });
        }

        Ok(AudioCommandResult {
            command_name: "Resume".to_string(),
            success: false,
            is_paused: player.is_paused(),
            path: None,
        })
    }
}

struct StopCommand {
    player: Arc<Mutex<Player>>,
}

impl StopCommand {
    fn new(player: Arc<Mutex<Player>>) -> Self {
        StopCommand { player }
    }

    fn run(&mut self) -> Result<AudioCommandResult> {
        let player = self.player.lock().unwrap();

        println!("Stopping: {:?}", player);

        if !player.has_ended() {
            player.end_current().unwrap();

            return Ok(AudioCommandResult {
                command_name: "Stop".to_string(),
                success: true,
                is_paused: player.is_paused(),
                path: None,
            });
        }

        Ok(AudioCommandResult {
            command_name: "Stop".to_string(),
            success: false,
            is_paused: player.is_paused(),
            path: None,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct AudioCommandResult {
    pub command_name: String,
    pub success: bool,
    pub is_paused: bool,
    pub path: Option<String>,
}

pub async fn handle_audio_command(
    app_handle: tauri::AppHandle,
    command: AudioCommands,
    play_params: Option<String>,
) -> Result<AudioCommandResult> {
    
    let player = Arc::new(Mutex::new(Player::new(
        String::from("Audio Player"),
        1.0,
        1.0,
    )));

    println!("handle_audio_command: {:?}", player);

    match command {
        AudioCommands::Play => {
            if let Some(params) = play_params {
                let mut play_command = PlayCommand::new(player.clone());
                let app = app_handle.clone();
                match play_command.run(app, PathBuf::from(params)) {
                    Ok(result) => Ok(result),
                    Err(error) => Err(error),
                }
            } else {
                Err(anyhow!("No params provided for play command."))
            }
        }
        AudioCommands::Pause => {
            let mut pause_command = PauseCommand::new(player.clone());
            match pause_command.run() {
                Ok(result) => Ok(result),
                Err(error) => Err(error),
            }
        }
        AudioCommands::Resume => {
            let mut resume_command = ResumeCommand::new(player.clone());
            match resume_command.run() {
                Ok(result) => Ok(result),
                Err(error) => Err(error),
            }
        }
        AudioCommands::Stop => {
            let mut stop_command = StopCommand::new(player.clone());
            match stop_command.run() {
                Ok(result) => Ok(result),
                Err(error) => Err(error),
            }
        }
    }
}
