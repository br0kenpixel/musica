use crate::dialog;
use homedir::get_my_home;
use std::path::PathBuf;

pub fn app_dir() -> PathBuf {
    home_dir().join("Musica")
}

pub fn config_path() -> PathBuf {
    app_dir().join("settings.json")
}

pub fn library_path() -> PathBuf {
    app_dir().join("Library")
}

fn home_dir() -> PathBuf {
    match get_my_home() {
        Ok(Some(path)) => path,
        Ok(None) => {
            dialog::error("Failed to get your home directory's path (unknown error)");
        }
        Err(why) => {
            dialog::error(format!("Failed to get your home directory's path ({why})"));
        }
    }
}
