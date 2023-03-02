#![allow(dead_code)]

use crate::{
    helpers::{configuration::{self}, self, player::AudioFileTypes},
};
use serde::{Deserialize, Serialize};

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
pub async fn set_app_config(
    app_handle: tauri::AppHandle,
    audio_directories: Vec<String>,
    audio_file_types_allowed: Vec<String>,
) -> bool {
    let config = configuration::AppConfig {
        audio_directories,
        audio_file_types_allowed,
    };
    match configuration::update_config_file(&app_handle, &config).await {
        Ok(_) => {
            return true;
        }
        Err(_) => {
            return false;
        }
    }
}

#[tauri::command]
pub async fn get_audio_files(app_handle: tauri::AppHandle, audio_file_type: String) -> Vec<String> {
    let config = configuration::read_config_file(app_handle).unwrap();
    let mut audio_files: Vec<String> = Vec::new();

    if config.audio_directories.len() == 0 {
        return audio_files;
    }

    if config.audio_file_types_allowed.len() == 1 {
        let files = helpers::player::get_audio_files(
            &config.audio_directories[0],
            AudioFileTypes::from_extension(&audio_file_type).unwrap(),
        );

        for file in files {
            audio_files.push(file.display().to_string());
        }

        return audio_files;
    }

    for directory in config.audio_directories {
        let files = helpers::player::get_audio_files(
            &directory,
            AudioFileTypes::from_extension(&audio_file_type).unwrap(),
        );

        for file in files {
            audio_files.push(file.display().to_string());
        }
    }
    return audio_files;
}

#[tauri::command]
pub async fn handle_audio_input() {
    
}

#[derive(Serialize, Deserialize)]
pub struct AppInfo {
    os: String,
    name: String,
    version: String,
    description: String,
}

#[tauri::command]
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
