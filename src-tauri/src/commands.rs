#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::App;

use crate::{
    config::AppConfig,
    types::{AppInfo, ConfigResult, TauriCommandError, AudioCommands, AudioCommandResult},
    utils::{self, AudioFileTypes}, manager::handle_audio_command,
};

#[tauri::command]
#[specta::specta]
pub async fn load_config(app_handle: tauri::AppHandle) -> Result<ConfigResult, String> {
    match AppConfig::new().load(app_handle) {
        Ok(config) => Ok(config),
        Err(e) => Err(e.message),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn save_config(
    app_handle: tauri::AppHandle,
    mut config: AppConfig,
) -> Result<ConfigResult, String> {
    match config.save(app_handle, config.clone()) {
        Ok(config) => Ok(config),
        Err(e) => Err(e.message),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn reset_config(app_handle: tauri::AppHandle) -> Result<ConfigResult, String> {
    match AppConfig::new().reset(app_handle) {
        Ok(config) => Ok(config),
        Err(e) => Err(e.message),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_audio_files(
    app_handle: tauri::AppHandle,
    audio_file_type: AudioFileTypes,
) -> Vec<String> {
    let config = match AppConfig::new().load(app_handle) {
        Ok(config) => config,
        Err(_) => {
            return Vec::new();
        }
    };

    let mut audio_files: Vec<String> = Vec::new();

    if config.data.audio_directories.len() == 0 {
        return audio_files;
    }

    if config.data.audio_file_types_allowed.len() == 1 {
        let files = utils::get_audio_files(&config.data.audio_directories[0], audio_file_type);

        for file in files {
            audio_files.push(file.display().to_string());
        }

        return audio_files;
    }

    for directory in config.data.audio_directories {
        let files = utils::get_audio_files(&directory, audio_file_type.clone());

        for file in files {
            audio_files.push(file.display().to_string());
        }
    }
    return audio_files;
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
            return Err(e.message);
        }
    }
}