use std::{
    fs,
    net::{TcpListener,TcpStream},
    io::{prelude::*, BufReader},
    string::ParseError,
};
pub struct Config{
    port: i32, // user-defined port <- used for localhost address (127.0.0.1)
    url: String // path for the file that is located within the users system
}

pub fn create_config() -> Result<(), ParseError>{
    // we take the user inputted values and store it into a struct
    // we store the struct data into a text file
    // we break down the struct data into a readable format like json or something
    // or we can make our own format like the below
    /*
        User Input: Port:3434 URL: C:/Desktop/Test/test.html 
        Struct Data: let config1 = Config{3434,"C:/Desktop/Test/test.html"}; <- according to the rust compiler
            
        How we store it in our own format
        Config1(
            Port: 3434, 
            URL: "C:/Desktop/Test/test.html"
        );
    */
    let address = "127.0.0.1:";
    let mut port = String::new();
    println!("Enter Port No::-> ");
    let port_byte_size = std::io::stdin().read_line(&mut port).unwrap();

    let mut path_url = String::new();
    println!("Enter Page File Path::-> ");
    let path_url_byte_size  = std::io::stdin().read_line(&mut path_url).unwrap();
    
    let full_addr = format!("{}{}",address,port);
    let _tcp = match full_addr.trim().parse::<String>(){
        Ok(tcp) => tcp,
        Err(e) => return Err(e)
    };

    
    println!("[port: {},bytes_read: {}]",port,port_byte_size);
    println!("[url: {},bytes_read: {}]",path_url,path_url_byte_size);
    println!("[Full_address: {}]",full_addr);

    
    Ok(())
}
    
pub fn read_config(){
    // we read our text file containing our configs
    // then we read the data in the file accordingly
    // we take the important information in the text file via parsing 
}
    
pub fn save_config(){
    // we call this function when user wants to save 
    // will be stored into a text file
}
    
pub fn load_config(){
    // we load all the configs saved in the text file
    // let user select one of the configs
    // then when user opens the page at the specified port
}    
    
pub fn default_config() -> i32{
    // load the default configuration
    // set by the program
    // port 3000
    // serves the hello.html page and 404.html page for errorinous condition
    let tcp_listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in tcp_listener.incoming(){
        let _stream = stream.unwrap();

        println!("Connection is Established!");
        default_handle_connection(_stream);
    }
    let c= Config { port: 3000, url: "".to_string() };
    return c.port;
}

fn default_handle_connection(mut connection_stream: TcpStream){
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