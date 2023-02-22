// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

mod audio_player;
mod helpers;
mod commands;

use tauri::api::shell;
use tauri::{CustomMenuItem, Manager, Menu, MenuEntry, Submenu};


fn main() {
    audio_player::core::init();
    let ctx = tauri::generate_context!();

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::view_app_config])
        .menu(Menu::with_items([
            #[cfg(target_os = "windows")]
            // Create a menu entry for the window home button
            MenuEntry::Submenu(Submenu::new(
                "Window",
                Menu::with_items(vec![
                    CustomMenuItem::new("Home", "Home").into(),
                    CustomMenuItem::new("App Configuration", "Settings").into(),
                    CustomMenuItem::new("About", "About").into(),
                ]),
            )),
            MenuEntry::Submenu(Submenu::new(
                "Help",
                Menu::with_items([
                    CustomMenuItem::new("Discord Server", "Discord").into(),
                    CustomMenuItem::new("Github Link", "Github").into(),
                ]),
            )),
        ]))
        .on_menu_event(|event| {
            let event_name = event.menu_item_id();
            match event_name {
                "Github Link" => {
                    let url = "https://github.com/ThatGuyJamal/Audio-Lion".to_string();
                    shell::open(&event.window().shell_scope(), url, None).unwrap();
                }
                "Discord Server" => {
                    let url = "https://discord.gg/MSTrBrNaGn".to_string();
                    shell::open(&event.window().shell_scope(), url, None).unwrap();
                }
                _ => {}
            }
        })
        .run(ctx)
        .expect("error while running tauri application");
}
