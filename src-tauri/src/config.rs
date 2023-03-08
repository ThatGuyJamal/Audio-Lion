#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use cpal::traits::{DeviceTrait, HostTrait};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::fs::{File, self};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io::{prelude::*, ErrorKind};
use std::path::Path;

use crate::types::{ConfigResult, IError, ISuccess};
use crate::utils::AudioFileTypes;

#[derive(Serialize, Deserialize, Debug, Type, Clone)]
/// The configuration file for the application
pub struct AppConfig {
    pub audio_directories: Vec<String>,
    // The name of the audio device to use for playback
    pub audio_device_name: Option<String>,
    // The file types to allow for playback
    pub audio_file_types_allowed: Vec<AudioFileTypes>,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            audio_directories: vec![],
            audio_device_name: None,
            audio_file_types_allowed: vec![AudioFileTypes::MP3, AudioFileTypes::WAV],
        }
    }

    /// Saves the configuration file to the specified path, if one exist it will be overwritten
    ///
    /// Returns `Ok(ConfigResult)` if the file was created successfully
    ///
    /// Returns `Err(IError)` if the file was not created successfully created successfully
    pub fn save(
        &mut self,
        app_handle: tauri::AppHandle,
        config: AppConfig,
    ) -> Result<ConfigResult, IError> {
        let config_json = match serde_json::to_string(&config) {
            Ok(json) => json,
            Err(e) => {
                return Err(IError {
                    status: false,
                    message: format!("Failed to serialize config to json: {}", e),
                })
            }
        };

        if let Some(config_path) = app_handle.path_resolver().app_config_dir() {
            // Create the directory if it doesn't exist
            match fs::create_dir_all(&config_path) {
                Ok(_) => (),
                Err(e) => {
                    return Err(IError {
                        status: false,
                        message: format!("Failed to create config directory: {}", e),
                    })
                }
            }

            // Write the config.json file to the config directory and handle errors
            let file_path = config_path.join("config.json");
            match std::fs::write(&file_path, config_json) {
                Ok(_) => {
                    return Ok(ConfigResult {
                        data: config,
                        error: None,
                    })
                }
                Err(e) => {
                    return Err(IError {
                        status: false,
                        message: format!("Failed to write config file: {}", e),
                    })
                }
            }
        } else {
            return Err(IError {
                status: false,
                message: "Failed to get config directory".to_string(),
            });
        }
    }

    /// Loads the configuration file from the specified path
    ///
    /// Returns `Ok(ConfigResult)` if the file was loaded successfully
    ///
    /// Returns `Err(IError)` if the file was not loaded successfully
    pub fn load(&mut self, app_handle: tauri::AppHandle) -> Result<ConfigResult, IError> {
        let config_file_path = app_handle
            .path_resolver()
            .app_config_dir()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unable to find writable config directory",
                )
            })
            .unwrap()
            .join("config.json");

        let mut config_file = match File::open(&config_file_path) {
            Ok(file) => file,
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    // Create the config file if it doesn't exist
                    let config = AppConfig::new();
                    return self.save(app_handle, config);
                } else {
                    return Err(IError {
                        status: false,
                        message: format!("Failed to open config file: {}", e),
                    });
                }
            }
        };

        let mut config_json = String::new();

        match config_file.read_to_string(&mut config_json) {
            Ok(_) => {
                let config: AppConfig = match serde_json::from_str(&config_json) {
                    Ok(config) => config,
                    Err(e) => {
                        return Err(IError {
                            status: false,
                            message: format!("Failed to deserialize config json: {}", e),
                        })
                    }
                };

                return Ok(ConfigResult {
                    data: config,
                    error: None,
                });
            }
            Err(e) => {
                return Err(IError {
                    status: false,
                    message: format!("Failed to read config file: {}", e),
                })
            }
        }
    }

    /// Resets the configuration file to the default values
    ///
    /// Returns `Ok(ConfigResult)` if the file was reset successfully
    ///
    /// Returns `Err(IError)` if the file was not reset successfully
    pub fn reset(&mut self, app_handle: tauri::AppHandle) -> Result<ConfigResult, IError> {
        let config = AppConfig::new();
        self.save(app_handle, config)
    }

    /// Checks if the configuration file exists
    pub fn exists(path_to_config: &str) -> bool {
        Path::new(&path_to_config).exists()
    }

    /// Returns a list of audio devices
    pub fn audio_directors(&self) -> Vec<String> {
        self.audio_directories.clone()
    }

    /// Returns the name of the audio device to use for playback
    pub fn audio_device_name(&self) -> Option<String> {
        self.audio_device_name.clone()
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}