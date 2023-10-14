use crate::{
    config::AppConfig,
    types::{AppError, AppInfo, ConfigResult, VideoLinkType},
    utils::{self, AudioFileTypes}, downloader::download_youtube_audio,
};

#[tauri::command]
#[specta::specta]
pub async fn load_config(app_handle: tauri::AppHandle) -> Result<ConfigResult, AppError> {
    match AppConfig::new().load(app_handle) {
        Ok(config) => Ok(config),
        Err(e) => Err(e),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn save_config(
    app_handle: tauri::AppHandle,
    config: AppConfig,
) -> Result<ConfigResult, AppError> {
    match AppConfig::new().save(app_handle, config.clone()) {
        Ok(config) => Ok(config),
        Err(e) => Err(e),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn reset_config(app_handle: tauri::AppHandle) -> Result<ConfigResult, AppError> {
    match AppConfig::new().reset(app_handle) {
        Ok(config) => Ok(config),
        Err(e) => Err(e),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_audio_files(
    app_handle: tauri::AppHandle,
    audio_file_type: AudioFileTypes,
) -> Result<Vec<String>, AppError> {
    let config = match AppConfig::new().load(app_handle) {
        Ok(config) => config,
        Err(_) => {
            return Err(AppError {
                status: false,
                message: "Failed to load config".to_string(),
            });
        }
    };

    let dirs = config.data.local_audio_folders;
    let mut audio_files: Vec<String> = vec![];

    // If no directories are set, return an empty vector
    if dirs.len() == 0 {
        return Ok(audio_files);
    }

    for dir in dirs {
        let files = utils::get_audio_files(&dir, audio_file_type.clone());

        if files.len() == 0 {
            continue;
        }

        for file in files {
            audio_files.push(file.display().to_string());
        }
    }

    return Ok(audio_files);
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
pub async fn download_audio_yt(url: &str, video_type: VideoLinkType, config: AppConfig)-> Result<(), AppError> {

    match download_youtube_audio(url, video_type, config) {
        Ok(_) => {},
        Err(e) => {
            return Err(e);
        }
    }

    Ok(())
}