// New, cleaner module structure
pub mod config;
pub mod http_handler;
pub mod server;
pub mod cli; // Renamed from webserve_cli

use cli::load; // Use the renamed module

pub fn main(){
    // This is the only thing main does.
    // It launches the Command Line Interface.
    load();
}