use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::process;
use std::process::{Stdio};

fn get_save_path() -> PathBuf {
    // Currently hardcoding text editor to nano for Linux and will hardcode Edit for Windows later.
    // Editor will be made configurable in the future.
    #[cfg(target_os = "linux")]
    {
        let home = env::var("HOME").unwrap();
        match env::var("XDG_DATA_HOME") {
            Ok(val) => {
                PathBuf::from(format!("{}/{}/oxnotes", home, val))
            }
            Err(_) => {
                PathBuf::from(format!("{}/.local/share/oxnotes", home))
            }
        }
    }
}

fn open_text_editor(path: &PathBuf) {
    #[cfg(target_os = "linux")]
    {
        process::Command::new("nano")
            .args([path.to_str().unwrap()])
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("Failed to open editor");
    }
}

fn get_last_path_item(path: &PathBuf) -> PathBuf {
    match path.components().last() {
        Some(comp) => PathBuf::from(comp.as_os_str()),
        None => {
            eprintln!("Error getting last path item");
            std::process::exit(1)
        },
    }
}

pub fn get_uncategorized_notes() -> Vec<PathBuf> {
    let mut entries: Vec<PathBuf> = Vec::new();
    let save_path = get_save_path();

    let read_dir = std::fs::read_dir(&save_path).unwrap();

    for entry in read_dir {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.parent() == Some(&save_path) {
            // EntryType::Category => {
            //     if path.is_dir() {
            //         entries.push(get_last_path_item(&path));
            //     }
            // }
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("note") {
                let mut filename = get_last_path_item(&path)
                    .to_string_lossy()
                    .to_string();
                if filename.ends_with(".note") {
                    filename.truncate(filename.len() - 5);
                }
                entries.push(PathBuf::from(filename));
            }
        }
    }

    entries
}

pub fn get_categorized_notes(category: &str) -> Vec<PathBuf> {
    let mut entries: Vec<PathBuf> = Vec::new();
    let save_path = get_save_path();
    let category_path = format!("{}/{}", save_path.to_str().unwrap(), category);

    let read_dir = match std::fs::read_dir(&category_path) {
        Ok(rd) => rd,
        Err(_) => {
            eprintln!("Category does not exist");
            std::process::exit(1);
        }
    };

    for entry in read_dir {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.parent() == Some(&PathBuf::from(&category_path)) {
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("note") {
                let mut filename = get_last_path_item(&path)
                    .to_string_lossy()
                    .to_string();
                if filename.ends_with(".note") {
                    filename.truncate(filename.len() - 5);
                }
                entries.push(PathBuf::from(filename));
            }
        }
    }

    entries
}

pub fn get_categories() -> Vec<PathBuf> {
    let mut entries: Vec<PathBuf> = Vec::new();
    let save_path = get_save_path();

    let read_dir = std::fs::read_dir(&save_path).unwrap();

    for entry in read_dir {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.parent() == Some(&save_path) {
            if path.is_dir() {
                entries.push(get_last_path_item(&path));
            }
        }
    }

    entries
}

pub fn create_category(name: &str) -> Result<PathBuf, std::io::Error> {
    let path = format!("{}/{}", get_save_path().to_str().unwrap(), name);

    match PathBuf::from(&path).exists() {
        false => {
            std::fs::create_dir_all(&path)?;
            Ok(PathBuf::from(path))
        }
        true => Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Category already exists")),
    }
}

pub fn create_uncategorized_note(name: &str) -> Result<PathBuf, std::io::Error> {
    let path = format!("{}/{}.note", get_save_path().to_str().unwrap(), name);

    match File::create_new(&path) {
        Ok(_) => {
            let pathbuf = PathBuf::from(&path);
            Ok(PathBuf::from(pathbuf))
        }
        Err(e) => Err(e),
    }
}

pub fn create_categorized_note(category: &str, name: &str) -> Result<PathBuf, std::io::Error> {
    let path = format!("{}/{}/{}.note", get_save_path().to_str().unwrap(), category, name);

    match PathBuf::from(&path).exists() {
        false => {
            File::create_new(&path)?;
            Ok(PathBuf::from(path))
        }
        true => Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Note already exists")),
    }
}

pub fn delete_category(name: &str) -> Result<(), std::io::Error> {
    let path = format!("{}/{}", get_save_path().to_str().unwrap(), name);

    match PathBuf::from(&path).exists() {
        true => {
            std::fs::remove_dir_all(&path)?;
            Ok(())
        }
        false => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Category does not exist")),
    }
}   

pub fn delete_uncategorized_note(name: &str) -> Result<(), std::io::Error> {
    let path = format!("{}/{}.note", get_save_path().to_str().unwrap(), name);

    match PathBuf::from(&path).exists() {
        true => {
            std::fs::remove_file(&path)?;
            Ok(())
        }
        false => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Note does not exist")),
    }
}

pub fn delete_categorized_note(category: &str, name: &str) -> Result<(), std::io::Error> {
    let path = format!("{}/{}/{}.note", get_save_path().to_str().unwrap(), category, name);

    match PathBuf::from(&path).exists() {
        true => {
            std::fs::remove_file(&path)?;
            Ok(())
        }
        false => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Note does not exist")),
    }
}

pub fn open_uncategorized_note(name: &str) -> Result<PathBuf, std::io::Error> {
    let path = format!("{}/{}.note", get_save_path().to_str().unwrap(), name);

    match PathBuf::from(&path).exists() {
        true => {
            let pathbuf = PathBuf::from(&path);
            open_text_editor(&pathbuf);
            Ok(PathBuf::from(pathbuf))
        }
        false => {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Note does not exist"));
        }
    }
}

pub fn open_categorized_note(category: &str, name: &str) -> Result<PathBuf, std::io::Error> {
    let path = format!("{}/{}/{}.note", get_save_path().to_str().unwrap(), category, name);

    match PathBuf::from(&path).exists() {
        true => {
            let pathbuf = PathBuf::from(&path);
            open_text_editor(&pathbuf);
            Ok(PathBuf::from(pathbuf))
        }
        false => {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Note does not exist"));
        }
    }
}