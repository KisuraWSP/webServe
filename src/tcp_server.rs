// "implement tcp server" that can be used anywhere when user runs executable
// run tcp related functions in a simplified macro for easy reuse

#[macro_export]
macro_rules! tcp_serve_default {
    ($host:expr, $port:expr) => {{
        let addr = format!("{}:{}",$host,$port);
        let tcp_listener = TcpListener::bind(addr).unwrap();
        // Create a thread pool with 4 threads
        let pool = ThreadPool::new(4);
    
        // .incoming() blocks until a new connection arrives.
        // This loop will run indefinitely, accepting new connections.
        for stream in tcp_listener.incoming(){
            let _stream = stream.unwrap();

            // Spawn a new thread from the pool to handle the connection
            // 'move' gives the closure ownership of '_stream'
            pool.execute(move || {
                println!("Connection is Established on a new thread!");

                // This closure defines the logic for handling a single connection
                let connection_handler = |mut connection_stream: TcpStream|{
                    let buffered_reader = BufReader::new(&mut connection_stream);
                    
                    // Read the first line (the request line)
                    // Added error handling to prevent panic on bad/empty request
                    let http_request_line = match buffered_reader.lines().next() {
                        Some(Ok(line)) => line,
                        Some(Err(_)) | None => {
                            println!("Failed to read HTTP request line.");
                            return; // Exit closure for this failed connection
                        }
                    };
                
                    let (status_line, filename) = if http_request_line == "GET / HTTP/1.1" {
                            ("HTTP/1.1 200 OK", "hello.html")
                        } else {
                            ("HTTP/1.1 404 NOT FOUND", "404.html")
                        };
                
                    // Read the file contents, with a fallback for 404.html
                    let contents = match fs::read_to_string(filename) {
                        Ok(content) => content,
                        Err(_) => {
                            println!("Could not read file: {}", filename);
                            // If 404.html is missing, send a hardcoded response
                            // Otherwise, send the contents of 404.html
                            if filename == "404.html" {
                                "<h1>404 Not Found</h1>".to_string()
                            } else {
                                // Try to read 404.html as a fallback
                                fs::read_to_string("404.html").unwrap_or_else(|_| "<h1>404 Not Found</h1>".to_string())
                            }
                        }
                    };

                    let length = contents.len();
            
                    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
            
                    // Write the response back to the stream and flush it
                    connection_stream.write_all(response.as_bytes()).unwrap_or_else(|e| println!("Failed to write response: {}", e));
                    connection_stream.flush().unwrap_or_else(|e| println!("Failed to flush stream: {}", e));
                };

                // Call the connection handler with the stream
                connection_handler(_stream);
            });
        }
    }};
}


// NEW MACRO for custom configs
#[macro_export]
macro_rules! tcp_serve_custom {
    ($host:expr, $port:expr, $file_path:expr) => {{
        let addr = format!("{}:{}", $host, $port);
        
        // --- FIX #2 ---
        // Replace the match block with .expect().
        // If binding fails, the program will panic and exit,
        // which solves the incompatible return type error.
        let tcp_listener = TcpListener::bind(&addr)
            .expect(&format!("Failed to bind to address {}", addr));

        let pool = ThreadPool::new(4);
        println!("Server listening on http://{}", addr);

        for stream in tcp_listener.incoming() {
            match stream {
                Ok(_stream) => {
                    // Clone the file path so it can be moved into the thread
                    let file_path_clone = $file_path.to_string();

                    pool.execute(move || {
                        println!("Connection established on custom config!");

                        let connection_handler = |mut connection_stream: TcpStream| {
                            let buffered_reader = BufReader::new(&mut connection_stream);
                            
                            let http_request_line = match buffered_reader.lines().next() {
                                Some(Ok(line)) => line,
                                Some(Err(_)) | None => {
                                    println!("Failed to read HTTP request line.");
                                    return;
                                }
                            };
                            
                            // --- FIX #1 ---
                            // Both arms of the if/else must return the same type.
                            // We make both return a String.
                            let (status_line, filename) = if http_request_line == "GET / HTTP/1.1" {
                                ("HTTP/1.1 200 OK", file_path_clone) // This is a String
                            } else {
                                ("HTTP/1.1 404 NOT FOUND", "404.html".to_string()) // This is now also a String
                            };

                            // fs::read_to_string takes AsRef<Path>, so we pass &filename
                            let contents = match fs::read_to_string(&filename) {
                                Ok(content) => content,
                                Err(_) => {
                                    println!("Could not read file: {}", filename);
                                    // Always try to read 404.html as a fallback
                                    fs::read_to_string("404.html")
                                        .unwrap_or_else(|_| "<h1>404 Not Found</h1><p>Additionally, the 404.html file is missing.</p>".to_string())
                                }
                            };

                            let length = contents.len();
                            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
                            
                            connection_stream.write_all(response.as_bytes()).unwrap_or_else(|e| println!("Failed to write response: {}", e));
                            connection_stream.flush().unwrap_or_else(|e| println!("Failed to flush stream: {}", e));
                        };

                        connection_handler(_stream);
                    });
                }
                Err(e) => {
                    eprintln!("Failed to establish connection: {}", e);
                }
            }
        }
    }};
}

// These are now replaced by tcp_serve_custom!
// We can remove the empty placeholders.
#[macro_export]
macro_rules! tcp_serve_new{
    () =>{{
        // This is now handled inside create_config() by calling tcp_serve_custom!
    }};
}

#[macro_export]
macro_rules! tcp_serve_load_existing{
    () => {
        // This is now handled inside load_config() by calling tcp_serve_custom!
    };
}