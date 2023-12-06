use std::{
    fs,
    fs::OpenOptions,
    net::{TcpListener,TcpStream},
    io::{prelude::*, BufReader,Write},
    string::ParseError,
};
struct Config{
    config_name: String,
    config_port: i32, // user-defined port <- used for localhost address (127.0.0.1)
    config_url: String // path for the file that is located within the users system
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
        How the representation means in our config file
        Config1(
            Port: 3434, 
            URL: "C:/Desktop/Test/test.html"
        );

        What it will look like as the final output
        Config1(3434,C:/Desktop/Test/test.html)
    */
    let address = "127.0.0.1:";
    let mut port = String::new();
    println!("Enter Port No::-> ");
    let port_byte_size = std::io::stdin().read_line(&mut port).unwrap();

    let mut path_url = String::new();
    println!("Enter Page File Path::-> ");
    let path_url_byte_size  = std::io::stdin().read_line(&mut path_url).unwrap();
    
    let mut name =  String::new();
    println!("Enter the Configuration Name::-> ");
    let name_byte_size = std::io::stdin().read_line(&mut name).unwrap();

    let full_addr = format!("{}{}",address,port);
    let _tcp = match full_addr.trim().parse::<String>(){
        Ok(tcp) => tcp,
        Err(e) => return Err(e)
    };

    let config = Config{
        config_name: name.trim().to_string().clone(),
        config_port: port.trim().parse::<i32>().unwrap(),
        config_url: path_url.trim().to_string().clone()
    };
    
    let mut log_file = OpenOptions::new().append(true).create(true).open("log.txt").expect("Open of Log File Failed");
     // need to implement a way to read the location of the file within the users entire system
     // for now it will just create the file where the executable lives

    log_file.write_all(format!("[config_name:{},bytes_read: {}]\n",name, name_byte_size).as_bytes()).expect("Write to Log file Failed");
    log_file.write_all(format!("[port: {},bytes_read: {}]\n",port,port_byte_size).as_bytes()).expect("Write to Log file Failed");
    log_file.write_all(format!("[url: {},bytes_read: {}]\n",path_url,path_url_byte_size).as_bytes()).expect("Write to Log file Failed");
    log_file.write_all(format!("[Full_address: {}]\n",full_addr).as_bytes()).expect("Write to Log file Failed");
    println!("Write to Log file...");

    let save_config = ||{
        // we call this function when user wants to save 
        // will be stored into a text file
        let mut config_file = OpenOptions::new().append(true).create(true).open("config.txt").expect("Open of Config File Failed");
        let saved_config = format!("{}({},{})\n",config.config_name,config.config_port,config.config_url).to_string();
        let _ = config_file.write_all(saved_config.as_bytes());
        println!("Write to Config File...");
    };
    save_config();
    Ok(())

    // Run Created Config
}
    
pub fn load_config(){
    // we load all the configs saved in the text file
    // let user select one of the configs
    // then when user opens the page at the specified port
    let read_config = ||{
        // we read our text file containing our configs
        // then we read the data in the file accordingly
        // we take the important information in the text file via parsing 
    };
    read_config();
}    
    
pub fn default_config(){
    // load the default configuration
    // set by the program
    // port 3000
    // serves the hello.html page and 404.html page for errorinous condition
    // Multithreading for tcp connection is required 
    // Indefinate Connection 
    let tcp_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    
    for stream in tcp_listener.incoming(){
        let _stream = stream.unwrap();

        println!("Connection is Established!");
        let connection_handler = |mut connection_stream: TcpStream|{
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
        };
        connection_handler(_stream);
    }
}
