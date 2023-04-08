#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{sync::{Arc, Mutex}, io::sink};

use specta::collect_types;
use tauri::Manager;
use tauri_specta::ts;
use types::Payload;
use window_shadows::set_shadow;

mod audio;
mod commands;
mod config;
mod types;
mod utils;

fn main() {
    // ! This must be disabled when building the app or it will not start.
    // export_bindings();

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
            commands::handle_audio,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
            commands::handle_audio
        ],
        "../src/lib/bindings.ts",
    ) {
        Ok(_) => {
            println!("Bindings exported successfully!");
        }
        Err(e) => {
            println!("Error exporting bindings: {}", e);
        }
    }
}
