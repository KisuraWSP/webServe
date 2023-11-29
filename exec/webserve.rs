use std::env;

fn help(){
    println!("LIST OF FLAGS")

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
			println!("{:?}",c);
        }
        else if command == "--load_config"{

        }
        else if command == "--default_config"{

        }
        else if command == "--exit"{

        }
        else{
            eprintln!("Command {command} doesnt exist!");
        }
        println!("Your argument {}", command);
    }
}
