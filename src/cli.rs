use ansi_term::{Colour::*, Style};
use std::env;
use std::io::Write;

// Import the functions from our new modules
use crate::config::{self, Config};
use crate::server;

// --- CLI Argument Parsing ---
macro_rules! args{
    () =>{{
        let args: Vec<String> = env::args().collect();
        if let Some(command) = args.get(1){
            match command.as_str() {
                "--help" => help!(),
                "--run" => app(), // Launch the beautiful UI
                _ => {
                    // Handle other flags or just launch the app
                    println!("Unknown command: {}", command);
                    println!("Use '--help' for commands or run with no args to launch.");
                    app(); // Default to launching the app
                }
            }
        } else {
            // No arguments provided, launch the app
            app();
        }
    }};
}

macro_rules! help {
    () => {{
        let title = Style::new().bold().paint("WebServe v0.2.0 (Refactored)");
        println!("{}", title);
        println!("\tA CLI utility for running static web servers.\n");
        println!("{}", Style::new().bold().underline().paint("Usage:"));
        println!("\twebserve [COMMAND]\n");
        println!("{}", Style::new().bold().underline().paint("Commands:"));
        println!("\t--run\t\tLoads the interactive WebServe CLI application.");
        println!("\t--help\t\tShows this help message.");
        println!("\nRun without commands to launch the interactive CLI directly.");
        ()
    }};
}

// --- Main Application UI ---

fn print_header() {
    let layer1 = "-------------------------------------------------------------------------------";
    let layer2 = "\\              / |----- |----|   |-------- |----- |------ \\      /  |------    ";
    let layer3 = " \\            /  |      |     |  |         |      |     |  \\    /   |          ";
    let layer4 = "  \\    /\\    /   |----- |----|   |-------| |----- |-----    \\  /    |------    ";
    let layer5 = "   \\__/  \\__/    |_____ |_____|  ________| |_____ |     |    \\/     |______    ";
    
    let style = Style::new().bold().on(Yellow).fg(Red);
    println!("\n{}", style.paint(layer1));
    println!("{}", style.paint(layer2));
    println!("{}", style.paint(layer3));
    println!("{}", style.paint(layer4));
    println!("{}", style.paint(layer5));
    println!("{}\n", style.paint(layer1));
}

fn get_user_choice() -> i8 {
    let mut option = String::new();
    std::io::stdin().read_line(&mut option).unwrap_or_default();
    option.trim().parse::<i8>().unwrap_or(-1) // Return -1 on bad parse
}

fn app() {
    let _ = ansi_term::enable_ansi_support();
    
    // Main application loop
    loop {
        print_header();
        println!("{}", Style::new().bold().paint("Select an option:"));
        println!("\t{} Load Default Server (Port 3000)", Cyan.paint("1:"));
        println!("\t{} Create New Static Server", Green.paint("2:"));
        println!("\t{} Load Existing Server", Yellow.paint("3:"));
        println!("\t{} Exit", Red.paint("4:"));
        print!("{}", Style::new().bold().paint("\n> "));
        std::io::stdout().flush().unwrap_or_default();

        let choice = get_user_choice();

        match choice {
            1 => run_default_server(),
            2 => run_new_server(),
            3 => run_load_server(),
            4 => {
                println!("\n{}", Cyan.paint("Thank you for using WebServe :)"));
                break; // Exit the loop
            },
            _ => {
                println!("\n{}", Red.paint("Invalid input. Please enter a number from 1 to 4."));
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
}

// --- UI Logic Functions ---

fn run_default_server() {
    println!("\n{}", Cyan.paint("Loading the Default Configuration..."));
    println!("View your page at -> http://127.0.0.1:3000/");
    
    // Get the default config struct
    let default_conf = config::get_default_config();
    
    // Tell the server module to run it
    server::run_server(default_conf, server::ServerType::Default);
}

fn run_new_server() {
    println!("\n{}", Green.paint("Creating a New Configuration..."));
    
    // config::create_config now just returns a config object
    match config::create_new_config() {
        Ok(new_config) => {
            println!("{}", Green.paint("Configuration saved!"));
            println!("View your page at -> http://127.0.0.1:{}/", new_config.port);

            // Tell the server module to run this new config
            server::run_server(new_config, server::ServerType::Custom);
        },
        Err(e) => {
            eprintln!("{}", Red.paint(format!("Failed to create config: {}", e)));
        }
    }
}

fn run_load_server() {
    println!("\n{}", Yellow.paint("Loading Existing Configurations..."));
    
    // Get all configs from the file
    let configs = match config::load_all_configs() {
        Ok(configs) => configs,
        Err(e) => {
            eprintln!("{}", Red.paint(format!("Failed to load configs: {}", e)));
            return;
        }
    };

    if configs.is_empty() {
        println!("{}", Red.paint("No configurations found in 'config.txt'."));
        return;
    }

    // Display options to user
    println!("Available Configurations:");
    for (i, cfg) in configs.iter().enumerate() {
        println!("  {}: {} (Port: {}, Path: {})", 
            Style::new().bold().paint((i + 1).to_string()), 
            cfg.name, 
            cfg.port, 
            cfg.file_path
        );
    }
    
    // Get user choice
    print!("{}", Style::new().bold().paint("\nEnter the number of the config to load: "));
    std::io::stdout().flush().unwrap_or_default();
    let choice = get_user_choice() as usize;

    if choice > 0 && choice <= configs.len() {
        let selected_config = &configs[choice - 1]; // Adjust for 0-based index
        
        println!("\nLoading config: {}", Style::new().bold().paint(&selected_config.name));
        println!("View your page at -> http://127.0.0.1:{}/", selected_config.port);
        
        // Tell the server module to run the selected config
        server::run_server(selected_config.clone(), server::ServerType::Custom);
    } else {
        println!("{}", Red.paint("Invalid selection. Returning to menu."));
    }
}

// Public entry point for main.rs
pub fn load(){
    args!();
}