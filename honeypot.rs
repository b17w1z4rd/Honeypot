use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs::OpenOptions;
use std::thread;

fn main() -> std::io::Result<()> {
    // Bind to the specified IP and port
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    println!("Honeypot listening on port 8080...");

    // Accept incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream)); // Handle the client in a new thread
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];

    // Read data from the client
    match stream.read(&mut buffer) {
        Ok(size) => {
            let data = String::from_utf8_lossy(&buffer[..size]);
            println!("Received data: {}", data);
            log_to_file(&data); // Log the data to a file

            // Respond to the client
            let response = "HTTP/1.1 200 OK\r\n\r\nWelcome to the honeypot!";
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => {
            eprintln!("Failed to read from socket: {}", e);
        }
    }
}

fn log_to_file(data: &str) {
    // Open or create a log file
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("honeypot_log.txt")
        .unwrap();

    // Write the received data to the log file
    if let Err(e) = writeln!(file, "{}", data) {
        eprintln!("Failed to write to log file: {}", e);
    }
}
