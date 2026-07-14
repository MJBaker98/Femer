use crate::file_utils::file_io::expand_tilde;
use dirs;
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
        // if let Some(xdg_home) = dirs::config_dir() {
        //     Self {
        //         config_dir: xdg_home,
        //     }
        // } else {
        //     panic!("Unable to read config home")
        // }
        if let Some(mut home_dir) = dirs::home_dir() {
            home_dir.push(".config");
            Self {
                config_dir: home_dir,
            }
        } else {
            panic!()
        }
    }

    pub fn get_config_dir(&self) -> Option<&str> {
        self.config_dir.to_str()
    }

    pub fn print_config_dir(&self) {
        println!("{}", self.config_dir.display())
    }
}
