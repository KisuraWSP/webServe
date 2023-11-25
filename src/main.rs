mod config;

use ansi_term::*;
use config::*;
use std::{
    fs,
    net::{TcpListener,TcpStream},
    io::{prelude::*, BufReader},
    string::ParseError,
};

fn main(){
    //default_config();
    let title = format!("{}",Style::new().bold().paint("WebServe"));
    println!("{}",title);
}
