#![allow(dead_code)]

use crate::{
    helpers::{
        self,
        configuration::{self, AppConfig},
        player::AudioFileTypes,
    },
    manager::{handle_audio_command, AudioCommandResult, AudioCommands}
};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, thiserror::Error)]
pub enum AudioCommandResultError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

// we must manually implement serde::Serialize
impl serde::Serialize for AudioCommandResultError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[tauri::command]
#[specta::specta]
pub async fn view_app_config(
    app_handle: tauri::AppHandle,
) -> Result<configuration::AppConfig, String> {
    match configuration::read_config_file(app_handle) {
        Ok(config) => {
            return Ok(config);
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn reset_app_config(app_handle: tauri::AppHandle) -> bool {
    match configuration::delete_config_file(&app_handle).await {
        // If the configuration file was deleted successfully, create a new one
        true => {
            // Clone app_handle and pass it to create_config_file
            let app_handle_clone = app_handle.clone();
            match configuration::create_config_file(app_handle_clone).await {
                Ok(_) => {
                    return true;
                }
                Err(_) => {
                    return false;
                }
            }
        }
        false => {
            return false;
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn set_app_config(
    app_handle: tauri::AppHandle,
    audio_directories: Vec<String>,
    audio_file_types_allowed: Vec<AudioFileTypes>,
    audio_device_name: Option<String>
) -> Result<AppConfig, String> {
    let config = AppConfig {
        audio_directories,
        audio_file_types_allowed,
        audio_device_name: audio_device_name
    };
    match configuration::update_config_file(&app_handle, &config).await {
        Ok(_) => {
            return Ok(config);
        }
        Err(_) => {
            return Err("Error updating config file".to_string());
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_audio_files(app_handle: tauri::AppHandle, audio_file_type: AudioFileTypes) -> Vec<String> {
    let config = configuration::read_config_file(app_handle).unwrap();
    let mut audio_files: Vec<String> = Vec::new();

    if config.audio_directories.len() == 0 {
        return audio_files;
    }

    if config.audio_file_types_allowed.len() == 1 {
        let files = helpers::player::get_audio_files(
            &config.audio_directories[0],
            audio_file_type,
        );

        for file in files {
            audio_files.push(file.display().to_string());
        }

        return audio_files;
    }

    for directory in config.audio_directories {
        let files = helpers::player::get_audio_files(
            &directory,
            audio_file_type.clone(),
        );

        for file in files {
            audio_files.push(file.display().to_string());
        }
    }
    return audio_files;
}

#[tauri::command]
#[specta::specta]
pub async fn handle_audio_input(
    app_handle: tauri::AppHandle,
    command: AudioCommands,
    player_path: Option<String>,
) -> Result<AudioCommandResult, String> {
    // println!("Command: {:?}", command);
    // println!("Player Path: {:?}", player_path);
    match handle_audio_command(app_handle, command, player_path).await {
        Ok(result) => {
            return Ok(result);
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

#[derive(Serialize, Deserialize, Type)]
pub struct AppInfo {
    os: String,
    name: String,
    version: String,
    description: String,
}

#[tauri::command]
#[specta::specta]
pub async fn get_app_info(app_handle: tauri::AppHandle) -> AppInfo {
    let package_info = app_handle.package_info();
    let os = std::env::consts::OS.to_string();
    let name = package_info.name.to_string();
    let version = package_info.version.to_string();
    let description = package_info.description.to_string();

    return AppInfo {
        os,
        name,
        version,
        description,
    };
}
