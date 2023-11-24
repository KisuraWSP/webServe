pub mod config;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    string::ParseError,
};
fn main() -> Result<(), ParseError>{
    let address = "127.0.0.1:";
    let mut port = String::new();
    println!("Enter Port No::-> ");
    let port_byte_size = std::io::stdin().read_line(&mut port).unwrap();

    let mut path_url = String::new();
    println!("Enter Page File Path::-> ");
    let path_url_byte_size  = std::io::stdin().read_line(&mut path_url).unwrap();
    
    let full_addr = format!("{}{}",address,port);
    let tcp = match full_addr.trim().parse::<String>(){
        Ok(tcp) => tcp,
        Err(e) => return Err(e)
    };

    
    println!("[port: {},bytes_read: {}]",port,port_byte_size);
    println!("[url: {},bytes_read: {}]",path_url,path_url_byte_size);
    println!("[Full_address: {}]",full_addr);

    let tcp_listener = TcpListener::bind(tcp).unwrap();

    for stream in tcp_listener.incoming(){
        let _stream = stream.unwrap();

        println!("Connection is Established!");
        handle_connection(_stream);
    }
    Ok(())
}

fn handle_connection(mut connection_stream: TcpStream){
    let buffered_reader = BufReader::new(&mut connection_stream);
    let http_request = buffered_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if http_request == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        connection_stream.write_all(response.as_bytes()).unwrap();                                                                                                                            
}