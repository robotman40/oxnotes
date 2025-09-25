use std::path::PathBuf;
use std::fs;
use std::env;

pub fn create_directory() {
    #[cfg(target_os = "linux")]
    {
        let home = env::var("HOME").unwrap();
        let path = match env::var("XDG_DATA_HOME") {
            Ok(val) => {
                PathBuf::from(format!("{}/{}/oxnotes", home, val))
            }
            Err(_) => {
                
                PathBuf::from(format!("{}/.local/share/oxnotes", home))
            }
        };

        if !path.exists() {
            match fs::create_dir_all(&path) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error creating directory: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
    #[cfg(target_os = "windows")]
    {
        let userprofile = env::var("USERPROFILE").unwrap();
        let path = PathBuf::from(format!("{}/AppData/Roaming/oxnotes", userprofile));

        if !path.exists() {
            match fs::create_dir_all(&path) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error creating directory: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}