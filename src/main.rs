mod config;

use ansi_term::{
    Colour::*,
    Style
};

use crate::config::{
    default_config, 
    create_config, 
    load_config
};

fn main(){
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
            println!("Loading the Default Configuration");
            default_config();
        } ,
        2=> {
            println!("Creating a New Configuration");
            let _ = create_config();
        },
        3=> {
            load_config();
        },
        4=> {
            
        },
        _=> print!("Invalid Option!!!")
    }
}
