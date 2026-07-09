

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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    match open_file(file_path) {
        Ok(contents) => {
            println!("Contents of {file_path}:");
            println!("{contents}");
        }
        Err(e) => {
            eprintln!("Error reading file '{file_path}': {e}");
            std::process::exit(1);
        }
    }
}
