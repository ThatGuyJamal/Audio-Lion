// #![allow(dead_code)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]

use crate::{
    config::AppConfig,
    types::{AppInfo, ConfigResult},
    utils::{self, AudioFileTypes},
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
    config: AppConfig,
) -> Result<ConfigResult, String> {
    match AppConfig::new().save(app_handle, config.clone()) {
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

    let mut audio_files: Vec<String> = vec![];

    // If no directories are set, return an empty vector
    if config.data.audio_directories.len() == 0 {
        return vec![];
    }

    for directory in config.data.audio_directories {
        let files = utils::get_audio_files(&directory, audio_file_type.clone());

        if files.len() == 0 {
            continue;
        }

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