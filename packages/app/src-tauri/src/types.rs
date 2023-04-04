#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use specta::Type;

// use crate::config::AppConfig;

/// Basic Success response to the front end.
#[derive(Serialize, Deserialize, Debug, Type)]
pub struct ISuccess {
    pub status: bool,
    pub message: Option<String>,
}

/// basic Errors returned by the application to the front end.
#[derive(Serialize, Deserialize, Debug, Type)]
pub struct IError {
    pub status: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Type)]
// The result of a config operation
pub struct ConfigResult {
    // pub data: AppConfig,
    pub error: Option<IError>,
}

#[derive(Clone, Serialize)]
pub struct Payload {
    pub args: Vec<String>,
    pub cwd: String,
}

#[derive(Serialize, Deserialize, Type)]
pub struct AppInfo {
    pub os: String,
    pub name: String,
    pub version: String,
    pub description: String,
}

/// Custom error type to json serialize string
///
/// see https://tauri.app/v1/guides/features/command#error-handling
#[derive(Debug, thiserror::Error)]
pub enum TauriCommandError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

// we must manually implement serde::Serialize
impl serde::Serialize for TauriCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string().as_ref())
    }
}

#[derive(Debug)]
pub enum AudioOutputError {
    OpenStreamError,
    PlayStreamError,
    StreamClosedError,
}

#[derive(Debug, Serialize, Deserialize, Type)]
/// Commands for the audio player to handle.
pub enum AudioCommands {
    Play,
    Pause,
    Resume,
    Stop,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct AudioCommandResult {
    pub command_name: String,
    pub success: bool,
    pub is_paused: bool,
    pub path: Option<String>,
}