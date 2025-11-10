use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
use std::path::{Path, PathBuf};

// --- DEFAULT SERVER HANDLER ---
// This is the logic from your old `tcp_serve_default!` macro
pub fn handle_default_connection(mut stream: TcpStream) {
    let buffered_reader = BufReader::new(&mut stream);
    let http_request_line = match buffered_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(_)) | None => {
            log_request("DEFAULT", "Failed to read request", "500");
            return;
        }
    };

    let (status_line, filename) = if http_request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    log_request("DEFAULT", &http_request_line, status_line);

    let contents = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(_) => {
            if filename == "404.html" {
                "<h1>404 Not Found</h1>".to_string()
            } else {
                fs::read_to_string("404.html").unwrap_or_else(|_| "<h1>404 Not Found</h1>".to_string())
            }
        }
    };

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap_or_default();
    stream.flush().unwrap_or_default();
}

// --- CUSTOM SERVER HANDLER ---
// This is the logic from your old `tcp_serve_custom!` macro
pub fn handle_custom_connection(mut stream: TcpStream, main_file: PathBuf, web_root: PathBuf) {
    let buffered_reader = BufReader::new(&mut stream);
    let http_request_line = match buffered_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(_)) | None => {
            log_request("CUSTOM", "Failed to read request", "500");
            return;
        }
    };

    // Get the absolute, "canonicalized" path for security checks
    let web_root_abs = match web_root.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            eprintln!("Failed to find web root directory: {:?}", web_root);
            return;
        }
    };
    let main_file_abs = match main_file.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            eprintln!("Failed to find main file: {:?}", main_file);
            return;
        }
    };

    let request_path = http_request_line.split(" ").nth(1).unwrap_or("/");

    let (status_line, file_to_read) = if request_path == "/" {
        ("HTTP/1.1 200 OK", main_file_abs)
    } else {
        let clean_path = request_path.strip_prefix("/").unwrap_or(request_path);
        let requested_file_path = web_root_abs.join(clean_path);

        match requested_file_path.canonicalize() {
            Ok(canonical_file_path) => {
                if !canonical_file_path.starts_with(&web_root_abs) {
                    ("HTTP/1.1 403 FORBIDDEN", PathBuf::from("404.html"))
                } else {
                    ("HTTP/1.1 200 OK", canonical_file_path)
                }
            }
            Err(_) => {
                // File not found, serve main file for client-side routing
                ("HTTP/1.1 200 OK", main_file_abs)
            }
        }
    };

    log_request("CUSTOM", &http_request_line, status_line);

    let (final_status, contents) = match fs::read(&file_to_read) {
        Ok(content) => (status_line.to_string(), content),
        Err(_) => {
            let status_404 = "HTTP/1.1 404 NOT FOUND".to_string();
            match fs::read("404.html") {
                Ok(content_404) => (status_404, content_404),
                Err(_) => (
                    status_404,
                    "<h1>404 Not Found</h1><p>And 404.html is missing.</p>".as_bytes().to_vec(),
                ),
            }
        }
    };

    let extension = file_to_read.extension().and_then(|s| s.to_str()).unwrap_or("");
    let content_type = get_mime_type(extension);
    let length = contents.len();

    let response_headers = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\nConnection: close\r\n\r\n",
        final_status, length, content_type
    );

    stream.write_all(response_headers.as_bytes()).unwrap_or_default();
    stream.write_all(&contents).unwrap_or_default();
    stream.flush().unwrap_or_default();
}

// --- HELPER FUNCTIONS ---

// Returns the correct MIME type for a file extension
fn get_mime_type(ext: &str) -> &str {
    match ext {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "eot" => "application/vnd.ms-fontobject",
        "mp4" => "video/mp4",
        "mp3" => "audio/mpeg",
        _ => "application/octet-stream", // Default binary type
    }
}

// Simple request logger
fn log_request(server_type: &str, request: &str, status: &str) {
    println!(
        "[{}] [{}] {} -> {}",
        chrono::Local::now().format("%H:%M:%S"),
        server_type,
        request,
        status
    );
}