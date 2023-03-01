#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use tauri::Manager;

mod audio_player;
mod commands;
mod helpers;

#[derive(Clone, Serialize)]
struct Payload {
  args: Vec<String>,
  cwd: String,
}

fn main() {
 tauri::Builder::default()
        .setup(|app| {
            audio_player::core::init(app);
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
            commands::play_audio_file,
            commands::pause_audio_file,
            commands::resume_audio_file,
            commands::stop_audio_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
