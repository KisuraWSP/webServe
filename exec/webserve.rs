use std::env;

fn help(){
    println!("LIST OF FLAGS");
    println!("--help\tDisplays List of Commands");
    println!("--hallo <Secondary Arguments...>\tDisplays a Message to the User");
    println!("--create_config\tCreates a new Network Configuration for the Server");
    println!("--load_config\tLoads an existing Configuration for the Server");
    println!("--default_config\tLoads the predefined Network Configuration for the Server");
    println!("--exit\tExits the Application");
}



fn main(){
    let args: Vec<String> = env::args().collect();
    if let Some(command) = args.get(1){
        if command == "--test"{
            println!("Test!");
            for arg in &args{
                println!("{}",arg);
            }
        }
        else if command == "--hallo"{
            println!("Hallo! {:?}",  &args[2..]);
        } 
        else if command == "--help"{
            help();
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
}
