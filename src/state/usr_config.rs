use crate::file_utils::file_io::expand_tilde;
use std::env;
use std::path::PathBuf;

/// Class which contains information on which applications are found on the computer
/// This class looks through the $XDG_CONFIG_HOME directory to find applications
///
/// # Arguments
///
/// * `argument_name` - type and description.
///
/// # Returns
/// type and description of the returned object.
///
/// # Examples
/// ```rust
/// write me later
/// ```
pub struct UserConfigDir {
    config_dir: PathBuf,
}

impl UserConfigDir {
    pub fn new() -> Self {
        let dir_path: PathBuf = PathBuf::new();
        if let Ok(xdg_home) = env::var("XDG_CONFIG_HOME") {
            let dir_path = PathBuf::from(xdg_home);
        } else {
            let dir_path = expand_tilde("~/.config");
        }
        Self {
            config_dir: dir_path,
        }
    }

    pub fn get_config_dir(&self) -> Option<&str> {
        self.config_dir.to_str()
    }

    pub fn print_config_dir(&self) {
        println!("{}", self.config_dir.display())
    }
}
