#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Imports
use config::AppConfig;
use specta::collect_types;
use tauri::{App, Manager};
use tauri_specta::ts;
use types::Payload;
use window_shadows::set_shadow;

use crate::utils::AudioFileTypes;

// Sub modules
mod api;
mod player;

mod commands;
mod config;
mod manager;
mod types;
mod utils;

// Start the application backend process
pub fn init(app: &mut App) {
    // Load the config file and make sure it exists
    match AppConfig::new().load(app.handle()) {
        Ok(_) => (),
        Err(e) => {
            println!("Error loading config on startup: {}", e.message);
            // Create the config file if it doesn't exist
            let defaults = AppConfig {
                audio_directories: vec![],
                audio_device_name: None,
                audio_file_types_allowed: vec![AudioFileTypes::MP3, AudioFileTypes::WAV],
            };
            match AppConfig::new().save(app.app_handle(), defaults) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error saving default config on startup: {}", e.message);
                }
            }
        }
    }
}

// Exports our rust types (converted to the similar js type) to a typescript file for use in the front end
fn export_bindings() {
    match ts::export(
        collect_types![
            commands::load_config,
            commands::save_config,
            commands::reset_config,
            commands::get_audio_files,
            commands::get_app_info,
            commands::handle_audio_input
        ],
        "../src/lib/bindings.ts",
    ) {
        Ok(_) => (),
        Err(e) => {
            println!("Error exporting bindings: {}", e);
        }
    }
}

fn main() {
    //! This must be disabled when building the app or it will not start.
    // export_bindings();

    tauri::Builder::default()
        .setup(|app| {
            init(app);
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            // println!("{}, {argv:?}, {cwd}", app.package_info().name);
            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();

            // Set the window shadow.
            let window = app.get_window("main").unwrap();
            match set_shadow(&window, true) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error setting window shadow: {}", e);
                }
            }
        }))
        .invoke_handler(tauri::generate_handler![
            commands::load_config,
            commands::save_config,
            commands::reset_config,
            commands::get_audio_files,
            commands::get_app_info,
            commands::handle_audio_input
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
