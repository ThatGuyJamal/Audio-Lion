#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use tauri::{Manager, App};

mod manager;
mod commands;
mod helpers;

mod audio;

#[derive(Clone, Serialize)]
struct Payload {
  args: Vec<String>,
  cwd: String,
}

fn main() {
 tauri::Builder::default()
        .setup(|app| {
            init(app);
            Ok(())
        })
         .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            // println!("{}, {argv:?}, {cwd}", app.package_info().name);
            app.emit_all("single-instance", Payload { args: argv, cwd }).unwrap();
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