#![allow(dead_code)]

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::App;

use crate::{types::{AudioCommandResult, AudioCommands, IError}, player::Player};


struct PlayCommand {
    player: Arc<Mutex<Player>>,
}

impl PlayCommand {
    fn new(player: Arc<Mutex<Player>>) -> Self {
        PlayCommand { player }
    }

    fn run(
        &mut self,
        app_handle: tauri::AppHandle,
        path: PathBuf,
    ) -> Result<AudioCommandResult, IError> {
        let mut player = self.player.lock().unwrap();

        println!("playing: {:?}", player);

        let path_clone = path.clone();

        if !player.has_ended() {
            match player.end_current() {
                Ok(_) => {
                    println!("Ended current track");
                }
                Err(e) => {
                    return Err(IError {
                        status: false,
                        message: format!("Error ending current track: {}", e),
                    })
                }
            }
        }

        match player.play_from_path(app_handle, path) {
            Ok(_) => {
                println!("Playing: {:?}", path_clone.display());
                return Ok(AudioCommandResult {
                    command_name: "play".to_string(),
                    success: true,
                    is_paused: false,
                    path: Some(path_clone.display().to_string()),
                });
            }
            Err(e) => {
                return Err(IError {
                    status: false,
                    message: format!("Error playing track: {}", e),
                })
            }
        }
    }
}

pub async fn handle_audio_command(
    app_handle: tauri::AppHandle,
    command: AudioCommands,
    play_params: Option<String>,
) -> Result<AudioCommandResult, IError> {
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
                    Ok(result) => {
                        println!("Player Thread {:?}", player);
                        Ok(result)
                    },
                    Err(error) => {
                        println!("Error: {:?}", error.message);
                        Err(error)
                    }
                }
            } else {
                Err(IError {
                    status: false,
                    message: "No path provided to play audio".to_string(),
                })
            }
        }
        AudioCommands::Pause => Ok(AudioCommandResult {
            command_name: "pause".to_string(),
            success: false,
            is_paused: true,
            path: None,
        }),
        AudioCommands::Stop => Ok(AudioCommandResult {
            command_name: "stop".to_string(),
            success: false,
            is_paused: false,
            path: None,
        }),
        AudioCommands::Resume => Ok(AudioCommandResult {
            command_name: "next".to_string(),
            success: false,
            is_paused: false,
            path: None,
        }),
    }
}
