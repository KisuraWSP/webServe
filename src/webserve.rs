pub mod config;
pub mod tcp_server;
pub mod webserve_cli;

use webserve_cli::load;
pub fn main(){
    load();
}