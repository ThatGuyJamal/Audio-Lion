#![allow(dead_code)]

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use lazy_static::lazy_static;

use crate::{
    player::Player,
    types::{AudioCommandResult, AudioCommands, IError},
};

// Global Vectors for storing the players and their ids. This is a bit of a hack, but it works for now.
// It allows us to control the players from the main thread, and the audio thread.
lazy_static! {
    pub static ref PLAYERS: Arc<Mutex<Vec<Arc<Mutex<Player>>>>> = Arc::new(Mutex::new(Vec::new()));
}

struct PlayCommand {
    player_id: usize,
    player: Arc<Mutex<Player>>,
}

impl PlayCommand {
    fn new(player_id: usize, player: Arc<Mutex<Player>>) -> Self {
        PlayCommand { player_id, player }
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

struct PauseCommand {
    player: Arc<Mutex<Player>>,
}

impl PauseCommand {
    fn new(player: Arc<Mutex<Player>>) -> Self {
        PauseCommand { player }
    }

    fn run(&mut self) -> Result<AudioCommandResult, IError> {
        let mut player = self.player.lock().unwrap();

        if !player.is_paused() && player.is_playing() {
            match player.pause() {
                Ok(_) => {
                    println!("Pausing: {:?}", player);

                    return Ok(AudioCommandResult {
                        command_name: "Pause".to_string(),
                        success: true,
                        is_paused: player.is_paused(),
                        path: None,
                    });
                }
                Err(e) => {
                    return Err(IError {
                        status: false,
                        message: format!("Error pausing track: {}", e),
                    })
                }
            }
        } else {
            return Err(IError {
                status: false,
                message: "No track to pause".to_string(),
            });
        }
    }
}

struct ResumeCommand {
    player: Arc<Mutex<Player>>,
}

impl ResumeCommand {
    fn new(player: Arc<Mutex<Player>>) -> Self {
        ResumeCommand { player }
    }

    fn run(&mut self) -> Result<AudioCommandResult, IError> {
        let mut player = self.player.lock().unwrap();

        if player.is_paused() {
            match player.unpause() {
                Ok(_) => {
                    println!("Resuming: {:?}", player);

                    return Ok(AudioCommandResult {
                        command_name: "Resume".to_string(),
                        success: true,
                        is_paused: player.is_paused(),
                        path: None,
                    });
                }
                Err(e) => {
                    return Err(IError {
                        status: false,
                        message: format!("Error resuming track: {}", e),
                    })
                }
            }
        } else {
            return Err(IError {
                status: false,
                message: "No track to resume".to_string(),
            });
        }
    }
}

struct StopCommand {
    player_id: usize,
    player: Arc<Mutex<Player>>,
}

impl StopCommand {
    fn new(player_id: usize, player: Arc<Mutex<Player>>) -> Self {
        StopCommand { player_id, player }
    }

    fn run(&mut self) -> Result<AudioCommandResult, IError> {
        let player = self.player.lock().unwrap();

        if !player.has_ended() {
            match player.end_current() {
                Ok(_) => {
                    println!("Ended current track");

                    // Remove the player from the global vector
                    PLAYERS.lock().unwrap().remove(self.player_id);

                    return Ok(AudioCommandResult {
                        command_name: "Stop".to_string(),
                        success: true,
                        is_paused: player.is_paused(),
                        path: None,
                    });
                }
                Err(e) => {
                    return Err(IError {
                        status: false,
                        message: format!("Error ending current track: {}", e),
                    })
                }
            }
        } else {
            return Err(IError {
                status: false,
                message: "No track to stop".to_string(),
            });
        }
    }
}

pub async fn handle_audio_command(
    app_handle: tauri::AppHandle,
    command: AudioCommands,
    play_params: Option<String>,
) -> Result<AudioCommandResult, IError> {
    // The ID of the player instance
    let player_id = std::time::Instant::now().elapsed().as_millis() as usize;

    // Create or get the player instance
    let player = {
        let mut players = PLAYERS.lock().unwrap();
        if player_id < players.len() {
            players[player_id].clone()
        } else {
            let new_player = Arc::new(Mutex::new(Player::new(
                format!("Audio Player {}", player_id),
                1.0,
                1.0,
            )));
            players.push(new_player.clone());
            new_player
        }
    };

    match command {
        AudioCommands::Play => {
            if let Some(params) = play_params {
                let mut play_command = PlayCommand::new(player_id.clone(), player.clone());
                let app = app_handle.clone();
                match play_command.run(app, PathBuf::from(params)) {
                    Ok(result) => {
                        println!("Player Thread {:?}", player);
                        Ok(result)
                    }
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
        AudioCommands::Pause => {
            let mut pause_command = PauseCommand::new(player.clone());
            match pause_command.run() {
                Ok(result) => {
                    println!("Player Thread {:?}", player);
                    Ok(result)
                }
                Err(error) => {
                    println!("Error: {:?}", error.message);
                    Err(error)
                }
            }
        }
        AudioCommands::Resume => {
            let mut resume_command = ResumeCommand::new(player.clone());
            match resume_command.run() {
                Ok(result) => {
                    println!("Player Thread {:?}", player);
                    Ok(result)
                }
                Err(error) => {
                    println!("Error: {:?}", error.message);
                    Err(error)
                }
            }
        }
        AudioCommands::Stop => {
            let mut stop_command = StopCommand::new(player_id, player.clone());
            match stop_command.run() {
                Ok(result) => {
                    println!("Player Thread {:?}", player);
                    Ok(result)
                }
                Err(error) => {
                    println!("Error: {:?}", error.message);
                    Err(error)
                }
            }
        }
    }
}
