pub mod core {
    use tauri::{Manager, App};

    use crate::helpers;

    // Invokes the Audio Resource Manager for Audio Lion
    pub fn init(app: &mut App) {
        println!("Audio Lion Core is now running!");

        // Load the configuration file
        load_config(app.app_handle());
    }

    fn load_config(app_handle: tauri::AppHandle) {
        // Load the configuration file
        match helpers::configuration::read_config_file(app_handle) {
            Ok(config) => {
                // Use the configuration data here
                println!("{:?}", config);
            }
            Err(e) => {
                // Handle the error here
                println!("Error: {:?}", e);
            }
        }
    }
}

pub mod stream {}