#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Imports
use config::AppConfig;
use serde::Serialize;
use specta::collect_types;
use tauri::{App, Manager};
use tauri_specta::ts;
use types::Payload;

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
    AppConfig::new().load(app.handle()).unwrap();
}

// Exports our rust types (converted to the similar js type) to a typescript file for use in the front end
fn export_bindings() {
    ts::export(
        collect_types![
            commands::load_config,
            commands::save_config,
            commands::reset_config,
            commands::get_audio_files,
            commands::get_app_info,
            commands::handle_audio_input
        ],
        "../src/lib/bindings.ts",
    )
    .unwrap();
}

fn main() {
    export_bindings();

    tauri::Builder::default()
        .setup(|app| {
            init(app);
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            // println!("{}, {argv:?}, {cwd}", app.package_info().name);
            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
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
