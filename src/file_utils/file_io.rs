use std::env;
use std::fs;
use std::path::PathBuf;

fn open_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let expanded_path = expand_tilde(path);
    let contents = fs::read_to_string(&expanded_path)?;
    Ok(contents)
}

fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") || path == "~" {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(path.replacen("~", &home, 1));
        }
    }
    PathBuf::from(path)
}
