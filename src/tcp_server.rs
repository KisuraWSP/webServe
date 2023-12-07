// "implement tcp server" that can be used anywhere when user runs executable
// run tcp related functions in a simplified macro for easy reuse

#[macro_export]
macro_rules! tcp_serve_default {
    ($host:expr, $port:expr) => {{
        let addr = format!("{}:{}",$host,$port);
        let tcp_listener = TcpListener::bind(addr).unwrap();
    
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
    }};
}

#[macro_export]
macro_rules! tcp_serve_new{
    () =>{{
        // creates a new config
        // gets user inputs regarding configs
        // immediately loads the new user config to the server
    }};
}

#[macro_export]
macro_rules! tcp_serve_load_existing{
    () => {
        // read the config.txt
        // gets the configurations
        // displays to user
        // user then selects what config they want to run
    };
}