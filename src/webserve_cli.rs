use ansi_term::{
    Colour::*,
    Style
};

use crate::config::{
    default_config, 
    create_config, 
    load_config
};

use std::env;

macro_rules! args{
    () =>{{
        let args: Vec<String> = env::args().collect();
        if let Some(command) = args.get(1){
            if command == "--test"{
                println!("Test!");
                for arg in &args{
                    println!("{}",arg);
                }
            }
            else if command == "--run-tests"{
                if cfg!(tests){
                    // to do later
                }
            }
            else if command == "--hallo"{
                println!("Hallo! {:?}",  &args[2..]);
            } 
            else if command == "--help"{
                help!();
            }
            else if command == "--run"{
                app();
            }
            else if command == "--app"{
                // loads the webserve gui interface
            }
            else if command == "--create_config"{
                println!("{}","creating new configuration");
                
            }
            else if command == "--load_config"{
                println!("{}","loading configuration options");
                
            }
            else if command == "--default_config"{
                println!("{}","loading default configuration");
                
            }
            else if command == "--exit"{
                println!("{}","exitting program");
            }
            else{
                eprintln!("Command {command} doesnt exist!");
            }
            println!("Your argument {}", command);
        }
    }};
}

macro_rules! help {
    () => {
        println!("LIST OF FLAGS");
        println!("--help\tDisplays List of Commands");
        println!("--hallo <Secondary Arguments...>\tDisplays a Message to the User");
        println!("--create_config\tCreates a new Network Configuration for the Server");
        println!("--run\tLoads the WebServe CLI Application");
        println!("--app\tLoads the WebServe GUI Application\t<- Coming Soon...");
        println!("--load_config\tLoads an existing Configuration for the Server");
        println!("--default_config\tLoads the predefined Network Configuration for the Server");
        println!("--test\tUsed to test your Command-Line Arguments");
        println!("--run-tests\t<- Coming Soon(To run the tests that are in the program)");
        println!("--exit\tExits the Application");
    };
}


fn app() {
    let _enabled = ansi_term::enable_ansi_support();
    let layer1 = "\t\t\t\t-------------------------------------------------------------------------------";
    let layer2 = "\t\t\t\t\\              / |----- |----|   |-------- |----- |------ \\      /  |------    ";
    let layer3 = "\t\t\t\t \\            /  |      |     |  |         |      |     |  \\    /   |          ";
    let layer4 = "\t\t\t\t  \\    /\\    /   |----- |----|   |-------| |----- |-----    \\  /    |------    ";
    let layer5 = "\t\t\t\t   \\__/  \\__/    |_____ |_____|  ________| |_____ |     |    \\/     |______    ";
    let layer6 = "\t\t\t\t-------------------------------------------------------------------------------";
    
    println!("{}",Style::new().bold().on(Yellow).fg(Red).paint(layer1));
    println!("{}",Style::new().bold().on(Yellow).fg(Red).paint(layer2));
    println!("{}",Style::new().bold().on(Yellow).fg(Red).paint(layer3));
    println!("{}",Style::new().bold().on(Yellow).fg(Red).paint(layer4));
    println!("{}",Style::new().bold().on(Yellow).fg(Red).paint(layer5));
    println!("{}",Style::new().bold().on(Yellow).fg(Red).paint(layer6));

    let mut option = String::new();

    println!("1: Load Default Configuration");
    println!("2: Create New Configuration");
    println!("3: Load Existing Configuration");
    println!("4: Exit");
    println!("SELECT OPTION!");
    let _option_reader = std::io::stdin().read_line(&mut option).unwrap();
    
    let choice:i8 = option.trim().parse::<i8>().unwrap();
    match choice {
        1=> {
            let default_msg = Green.paint("Loading the Default Configuration...").to_string();
            println!("{}",default_msg);
            default_config();
        } ,
        2=> {
            let create_msg = Purple.paint("Creating a New Configuration...").to_string();
            println!("{}",create_msg);
            let _ = create_config();
        },
        3=> {
            let load_msg = Yellow.paint("Loading Existing Configurations List...").to_string();
            println!("{}",load_msg);
            load_config();
        },
        4=> {
            let exit_msg = Cyan.paint("Thank For Using WebServe :)").to_string();
            println!("{}",exit_msg);
        },
        _=> {
            print!("{}",Red.paint("Invalid Input Entered!").to_string());
        }
    }
}

pub fn load(){
    args!();
}
