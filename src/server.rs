// Once and for all
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // TODO: Replace with BufReader.
    let mut buf = [0; 1024];
    stream.read(&mut buf).expect("Failed to read from client.");
    let request = String::from_utf8_lossy(&buf[..]);
    println!("Receiver request: {}", request);
    let response = "Hello, Client!".as_bytes();
    stream.write(response).expect("Failed to write response.");
}
// app: &mut App
pub fn run() {
    let conn = "0.0.0.0:8080";
    
    let listener = TcpListener::bind(conn).
    expect("Failed to bind address.");
    println!("Server listening on: {}", conn);
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e)  => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}