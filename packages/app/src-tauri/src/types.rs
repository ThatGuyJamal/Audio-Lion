#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::config::AppConfig;

// use crate::config::AppConfig;

/// Basic Success response to the front end.
#[derive(Serialize, Deserialize, Debug, Type)]
pub struct ISuccess {
    pub status: bool,
    pub message: Option<String>,
}

/// basic Errors returned by the application to the front end.
#[derive(Serialize, Deserialize, Debug, Type)]
pub struct AppError {
    pub status: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Type)]
// The result of a config operation
pub struct ConfigResult {
    pub data: AppConfig,
    pub error: Option<AppError>,
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

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
/// The User data information saved from discord oauth
pub struct AppUser {
    // Handled by discord api
    pub discord_id: String,
    pub access_token: String,
    pub refresh_token: String,

    // Handled by the application
    /// A name to display to the user in the app.
    pub display_name: Option<String>,
}
