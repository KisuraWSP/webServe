mod config;

use ansi_term::*;



fn main(){
    //default_config();
    let title = format!("{}",Style::new().bold().paint("WebServe"));
    println!("{}",title);
}
