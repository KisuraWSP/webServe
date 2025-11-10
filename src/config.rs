use std::{
    fs,
    fs::OpenOptions,
    net::{TcpListener,TcpStream},
    io::{prelude::*, BufReader,Write},
    string::ParseError,
    path::PathBuf, // <-- ADDED for path logic
};
// Import BOTH macros
use crate::{tcp_serve_default, tcp_serve_custom};
use threadpool::ThreadPool;

// Added derive(Debug, Clone) for easier parsing and handling
#[derive(Debug, Clone)]
struct Config{
    config_name: String,
    config_port: i32, // user-defined port <- used for localhost address (127.0.0.1)
    config_url: String // path for the file that is located within the users system
}

pub fn create_config() -> Result<(), ParseError>{
    let address = "127.0.0.1"; // Host is separate now
    let mut port = String::new();
    println!("Enter Port No::-> ");
    let port_byte_size = std::io::stdin().read_line(&mut port).unwrap();

    let mut path_url = String::new();
    println!("Enter Page File Path::-> ");
    let path_url_byte_size  = std::io::stdin().read_line(&mut path_url).unwrap();
    
    let mut name =  String::new();
    println!("Enter the Configuration Name::-> ");
    let name_byte_size = std::io::stdin().read_line(&mut name).unwrap();

    // Trim whitespace from inputs
    let port_trimmed = port.trim();
    let path_url_trimmed = path_url.trim();
    let name_trimmed = name.trim();

    let full_addr = format!("{}:{}", address, port_trimmed);
    let _tcp = match full_addr.trim().parse::<String>(){
        Ok(tcp) => tcp,
        Err(e) => return Err(e)
    };

    let config = Config{
        config_name: name_trimmed.to_string().clone(),
        config_port: port_trimmed.parse::<i32>().unwrap(),
        config_url: path_url_trimmed.to_string().clone()
    };
    
    let mut log_file = OpenOptions::new().append(true).create(true).open("log.txt").expect("Open of Log File Failed");
    
    log_file.write_all(format!("[config_name:{},bytes_read: {}]\n",name_trimmed, name_byte_size).as_bytes()).expect("Write to Log file Failed");
    log_file.write_all(format!("[port: {},bytes_read: {}]\n",port_trimmed,port_byte_size).as_bytes()).expect("Write to Log file Failed");
    log_file.write_all(format!("[url: {},bytes_read: {}]\n",path_url_trimmed,path_url_byte_size).as_bytes()).expect("Write to Log file Failed");
    log_file.write_all(format!("[Full_address: {}]\n",full_addr).as_bytes()).expect("Write to Log file Failed");
    println!("Write to Log file...");

    let save_config = ||{
        let mut config_file = OpenOptions::new().append(true).create(true).open("config.txt").expect("Open of Config File Failed");
        // Use the trimmed versions for saving
        let saved_config = format!("{}({},{})\n",config.config_name,config.config_port,config.config_url).to_string();
        let _ = config_file.write_all(saved_config.as_bytes());
        println!("Write to Config File...");
    };
    save_config();

    // --- MODIFIED THIS PART ---
    // We determine the "web root" (the parent directory)
    let main_file_path = PathBuf::from(&config.config_url);
    let web_root_path = main_file_path.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf();

    println!("Launching new server on http://{}:{}/", address, config.config_port);
    println!("Serving file: {:?}", main_file_path);
    println!("From web root: {:?}", web_root_path);

    // Call the NEW macro with the main file AND the web root
    // --- FIX: Pass references (&) to the PathBuf variables ---
    tcp_serve_custom!(address, config.config_port, &main_file_path, &web_root_path);

    Ok(())
}
    
pub fn load_config(){
    let mut configs: Vec<Config> = Vec::new();

    let file_content = match fs::read_to_string("config.txt") {
        Ok(content) => content,
        Err(_) => {
            println!("Could not find 'config.txt'. Please create a config first.");
            return;
        }
    };

    for line in file_content.lines() {
        if line.trim().is_empty() { continue; } // Skip empty lines

        // Parse format: ConfigName(Port,Path)
        if let Some((name, rest)) = line.split_once('(') {
            if let Some((data, _)) = rest.rsplit_once(')') {
                if let Some((port_str, path_str)) = data.split_once(',') {
                    if let Ok(port) = port_str.trim().parse::<i32>() {
                        let config = Config {
                            config_name: name.trim().to_string(),
                            config_port: port,
                            config_url: path_str.trim().to_string(),
                        };
                        configs.push(config);
                    }
                }
            }
        }
    }

    if configs.is_empty() {
        println!("No valid configs found in 'config.txt'.");
        return;
    }

    // 3. Display options to user
    println!("Available Configurations:");
    for (i, config) in configs.iter().enumerate() {
        println!("  {}: {} (Port: {}, Path: {})", i + 1, config.config_name, config.config_port, config.config_url);
    }

    // 4. Get user choice
    let mut choice_str = String::new();
    println!("Enter the number of the config to load:");
    std::io::stdin().read_line(&mut choice_str).expect("Failed to read line");

    match choice_str.trim().parse::<usize>() {
        Ok(choice) if choice > 0 && choice <= configs.len() => {
            let selected_config = &configs[choice - 1]; // Adjust for 0-based index
            
            // --- MODIFIED THIS PART ---
            // Determine web root from the loaded config path
            let main_file_path = PathBuf::from(&selected_config.config_url);
            let web_root_path = main_file_path.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf();
            
            println!("Loading config: {}", selected_config.config_name);
            println!("View your page at -> http://127.0.0.1:{}/", selected_config.config_port);
            println!("Serving file: {:?}", main_file_path);
            println!("From web root: {:?}", web_root_path);

            // Call the NEW macro with the main file AND the web root
            // --- FIX: Pass references (&) to the PathBuf variables ---
            tcp_serve_custom!("127.0.0.1", selected_config.config_port, &main_file_path, &web_root_path);
        }
        _ => {
            println!("Invalid selection. Please enter a number from 1 to {}.", configs.len());
        }
    }
}    
    
pub fn default_config(){
    // --- THIS IS UNCHANGED ---
    // It correctly calls the simple, original macro.
    tcp_serve_default!("127.0.0.1",3000);
}