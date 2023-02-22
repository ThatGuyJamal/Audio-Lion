pub mod core {
    use crate::helpers;

    // Invokes the Audio Resource Manager for Audio Lion
    pub fn init() {
        println!("Audio Lion Core is now running!");

        match helpers::configuration::read_config_file() {
            Ok(config) => {
                // Use the configuration data here
                println!("Loaded AppConfig: {:?}", config);
            }
            Err(e) => {
                eprintln!("Error reading config file: {}", e);
            }
        }
    }
}
