// Added path and PathBuf for file system routing
use std::path::{Path, PathBuf};

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
    // UPDATED signature to accept a main file and a web root directory
    ($host:expr, $port:expr, $main_file:expr, $web_root:expr) => {{
        let addr = format!("{}:{}", $host, $port);
        
        let tcp_listener = TcpListener::bind(&addr)
            .expect(&format!("Failed to bind to address {}", addr));

        let pool = ThreadPool::new(4);
        println!("Server listening on http://{}", addr);

        // --- NEW ---
        // Get the absolute, "canonicalized" path for security checks
        let root_path_buf = $web_root.to_path_buf();
        let web_root_abs = match root_path_buf.canonicalize() {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Failed to find web root directory {:?}: {}", root_path_buf, e);
                panic!("Web root not found.");
            }
        };
        // Also canonicalize the main file path
        let main_file_abs = match $main_file.to_path_buf().canonicalize() {
             Ok(path) => path,
             Err(e) => {
                eprintln!("Failed to find main file {:?}: {}", $main_file, e);
                panic!("Main file not found.");
            }
        };
        // --- END NEW ---

        for stream in tcp_listener.incoming() {
            match stream {
                Ok(_stream) => {
                    // Clone the paths to move into the thread
                    let main_file_clone = main_file_abs.clone();
                    let web_root_clone = web_root_abs.clone();

                    pool.execute(move || {
                        let connection_handler = |mut connection_stream: TcpStream| {
                            let buffered_reader = BufReader::new(&mut connection_stream);
                            
                            let http_request_line = match buffered_reader.lines().next() {
                                Some(Ok(line)) => line,
                                Some(Err(_)) | None => {
                                    println!("Failed to read HTTP request line.");
                                    return;
                                }
                            };

                            // --- NEW ROUTING LOGIC ---
                            // Get the requested path (e.g., "/", "/style.css", "/about")
                            let request_path = http_request_line
                                .split(" ")
                                .nth(1)
                                .unwrap_or("/");

                            // Determine which file to serve
                            let (status_line, file_to_read) = 
                                if request_path == "/" {
                                    // Request for root, serve main HTML file
                                    ("HTTP/1.1 200 OK", main_file_clone)
                                } else {
                                    // Request for another file (e.g., /style.css)
                                    // Clean the path to remove leading '/'
                                    let clean_path = request_path.strip_prefix("/").unwrap_or(request_path);
                                    let requested_file_path = web_root_clone.join(clean_path);

                                    // --- SECURITY CHECK & ROUTING ---
                                    match requested_file_path.canonicalize() {
                                        Ok(canonical_file_path) => {
                                            // Ensure the path is still INSIDE the web root
                                            if !canonical_file_path.starts_with(&web_root_clone) {
                                                // Directory traversal attempt!
                                                ("HTTP/1.1 403 FORBIDDEN", PathBuf::from("404.html"))
                                            } else {
                                                // File is safe and exists, serve it
                                                ("HTTP/1.1 200 OK", canonical_file_path)
                                            }
                                        }
                                        Err(_) => {
                                            // File not found OR path is bad
                                            // Serve main file to allow client-side routing
                                            ("HTTP/1.1 200 OK", main_file_clone)
                                        }
                                    }
                                };
                            
                            // --- NEW MIME TYPE and BINARY READ ---

                            // Get file extension for MIME type
                            let extension = file_to_read.extension().and_then(|s| s.to_str()).unwrap_or("");
                            let content_type = match extension {
                                "html" => "text/html",
                                "css" => "text/css",
                                "js" => "application/javascript",
                                "png" => "image/png",
                                "jpg" | "jpeg" => "image/jpeg",
                                "gif" => "image/gif",
                                // Add more types as needed
                                _ => "application/octet-stream", // Default binary type
                            };
                            
                            // Read file as raw BYTES, not string
                            let (final_status, contents) = match fs::read(&file_to_read) {
                                Ok(content) => (status_line.to_string(), content),
                                Err(_) => {
                                    // Primary file failed, try 404.html
                                    match fs::read("404.html") {
                                        Ok(content_404) => ("HTTP/1.1 404 NOT FOUND".to_string(), content_404),
                                        Err(_) => (
                                            // Hardcoded fallback if 404.html is also missing
                                            "HTTP/1.1 404 NOT FOUND".to_string(), 
                                            "<h1>404 Not Found</h1><p>And 404.html is missing.</p>".as_bytes().to_vec()
                                        )
                                    }
                                }
                            };

                            let length = contents.len();
                            // Build the response headers
                            let response_headers = format!(
                                "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
                                final_status, length, content_type
                            );
                            
                            // Write headers first, then binary content
                            connection_stream.write_all(response_headers.as_bytes()).unwrap_or_else(|e| println!("Failed to write headers: {}", e));
                            connection_stream.write_all(&contents).unwrap_or_else(|e| println!("Failed to write content: {}", e));
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