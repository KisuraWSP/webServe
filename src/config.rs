use std::{
    fs,
    fs::OpenOptions,
    io::{prelude::*, Error as IoError, ErrorKind, Write},
    path::PathBuf,
};

// Renamed fields for clarity
#[derive(Debug, Clone)]
pub struct Config {
    pub name: String,
    pub port: i32,
    pub file_path: String, // The main HTML file
    pub web_root: PathBuf,   // The directory to serve files from
}

// Helper to get user input safely
fn get_user_input(prompt: &str) -> Result<String, IoError> {
    print!("{}", prompt);
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

// Returns a default Config struct
// This is the function cli.rs is looking for
pub fn get_default_config() -> Config {
    Config {
        name: "Default".to_string(),
        port: 3000,
        file_path: "hello.html".to_string(),
        web_root: PathBuf::from("."), // Current directory
    }
}

// Prompts user, creates, saves, and returns a new Config
// This is the function cli.rs is looking for
pub fn create_new_config() -> Result<Config, Box<dyn std::error::Error>> {
    let port_str = get_user_input("Enter Port No::-> ")?;
    let port = port_str.parse::<i32>()?;
    
    let file_path_str = get_user_input("Enter Page File Path::-> ")?;
    let name = get_user_input("Enter the Configuration Name::-> ")?;

    let main_file = PathBuf::from(&file_path_str);
    let web_root = main_file.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf();

    let new_config = Config {
        name,
        port,
        file_path: file_path_str,
        web_root,
    };

    // Save the new config
    save_config_to_file(&new_config)?;
    
    Ok(new_config)
}

// Appends a single config to the config.txt file
fn save_config_to_file(config: &Config) -> Result<(), IoError> {
    let mut config_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("config.txt")?;
    
    let saved_config = format!(
        "{}({},{})\n",
        config.name, config.port, config.file_path
    );
    
    config_file.write_all(saved_config.as_bytes())?;
    
    // Also write to log.txt
    let mut log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")?;
    
    let log_entry = format!(
        "[{}] Config Saved: Name: {}, Port: {}, Path: {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        config.name,
        config.port,
        config.file_path
    );
    log_file.write_all(log_entry.as_bytes())?;

    println!("Write to Config File and Log File...");
    Ok(())
}

// Loads all configs from config.txt
// This is the function cli.rs is looking for
pub fn load_all_configs() -> Result<Vec<Config>, IoError> {
    let file_content = match fs::read_to_string("config.txt") {
        Ok(content) => content,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            // If no file, return an empty list. Not an error.
            return Ok(Vec::new()); 
        }
        Err(e) => return Err(e),
    };

    let mut configs: Vec<Config> = Vec::new();
    for line in file_content.lines() {
        if line.trim().is_empty() { continue; }

        if let Some((name, rest)) = line.split_once('(') {
            if let Some((data, _)) = rest.rsplit_once(')') {
                if let Some((port_str, path_str)) = data.split_once(',') {
                    if let Ok(port) = port_str.trim().parse::<i32>() {
                        
                        let main_file = PathBuf::from(path_str.trim());
                        let web_root = main_file.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf();

                        configs.push(Config {
                            name: name.trim().to_string(),
                            port,
                            file_path: path_str.trim().to_string(),
                            web_root,
                        });
                    }
                }
            }
        }
    }

    Ok(configs)
}

// --- THESE FUNCTIONS ARE REPLACED BY THE ONES ABOVE ---
// We keep them here but change cli.rs to call the new ones.
// Since you provided the old config.rs, I am replacing the
// whole file with the new one. This file no longer contains
// create_config(), load_config(), or default_config().