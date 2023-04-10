#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use types::Payload;
use window_shadows::set_shadow;
use utils::export_bindings;

mod commands;
mod types;
mod utils;
mod config;
mod downloader;
mod user;

fn main() {
    // ! This must be disabled when building the app or it will not start.
    export_bindings(true);

    tauri::Builder::default()
        .setup(|app| {
            // init(app);
            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            // Forces only one instance of the app to run at a time on the system.
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

            // println!("{}, {argv:?}, {cwd}", app.package_info().name);
        }))
        .invoke_handler(tauri::generate_handler![
            commands::load_config,
            commands::save_config,
            commands::reset_config,
            commands::get_audio_files,
            commands::get_app_info,
            commands::download_audio_yt
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}