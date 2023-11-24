use std::net::TcpListener;
fn main() {
    let address = "127.0.0.1:";
    let mut input = String::new();
    println!("Enter Port No::-> ");
    let port = std::io::stdin().read_line(&mut input).unwrap();

    let mut input2 = String::new();
    println!("Enter Page File Path::-> ");
    let url  = std::io::stdin().read_line(&mut input2).unwrap();
    
    println!("[port: {},bytes read size: {}]",input,port);
    println!("[url: {},bytes read size: {}]",input2,url);

    let tcp_listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in tcp_listener.incoming(){
        let _stream = stream.unwrap();

        println!("Connection is Established!");
    }
}
