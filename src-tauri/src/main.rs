#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use specta::collect_types;
use tauri::{App, Manager};
use tauri_specta::ts;

mod commands;
mod helpers;
mod manager;

mod audio;

#[derive(Clone, Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
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
            commands::view_app_config,
            commands::reset_app_config,
            commands::set_app_config,
            commands::get_audio_files,
            commands::get_app_info,
            commands::handle_audio_input,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn init(app: &mut App) {
    match helpers::configuration::read_config_file(app.handle()) {
        Ok(_config) => {}
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

fn export_bindings() {
    ts::export(
        collect_types![
            commands::view_app_config,
            commands::reset_app_config,
            commands::set_app_config,
            commands::get_audio_files,
            commands::get_app_info,
            // todo - Fix types for this command so they can be exported
            commands::handle_audio_input
        ],
        "../src/lib/bindings.ts",
    )
    .unwrap();
}
