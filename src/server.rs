use crate::config::Config;
use crate::http_handler; // Import the new handler module
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use threadpool::ThreadPool;

// An enum to tell the server which handler to use
pub enum ServerType {
    Default,
    Custom,
}

// This is the main server function. It takes a config and a type.
pub fn run_server(config: Config, server_type: ServerType) {
    let address = format!("127.0.0.1:{}", config.port);
    
    let listener = match TcpListener::bind(&address) {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to address {}: {}", address, e);
            return;
        }
    };
    
    println!("Server listening on http://{}", address);
    let pool = ThreadPool::new(4); // Create the thread pool here

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                // For custom servers, we need to clone the path data
                let main_file_clone = PathBuf::from(&config.file_path);
                let web_root_clone = config.web_root.clone();

                // Dispatch to the correct handler based on server type
                match server_type {
                    ServerType::Default => {
                        pool.execute(move || {
                            http_handler::handle_default_connection(_stream);
                        });
                    }
                    ServerType::Custom => {
                        pool.execute(move || {
                            http_handler::handle_custom_connection(
                                _stream, 
                                main_file_clone, 
                                web_root_clone
                            );
                        });
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}