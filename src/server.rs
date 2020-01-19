use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use rustyecho::get_address;

fn main() -> io::Result<()> {
    // Set server address
    let address = get_address();

    // Bind to address
    println!("Binding to {}", address);
    let listener = TcpListener::bind(address.to_string())?;
    println!("Successfully bound to {}, listening...", address);

    // Main loop
    for stream in listener.incoming() {
        thread::spawn(move || handle_stream(stream?));
    }

    Ok(())
}

fn handle_stream(mut stream: TcpStream) -> io::Result<()> {
    // Buffer to store remote input
    let mut buffer = [0; 512];

    loop {
        // Read bytes from the stream into buffer
        let bytes_read = match stream.read(&mut buffer) {
            Ok(n) if n == 0 => {
                // Client stream disconnected
                println!("Connection closed: {:?}", stream);
                return Ok(());
            }
            Ok(n) => n, // Return number of bytes n read
            Err(e) => {
                // An error occured
                eprintln!("Failed to read from stream: {:?}", e);
                return Err(e);
            }
        };

        // Sleep 100 milliseconds
        thread::sleep(Duration::from_millis(100));

        // Write (echo) buffer back into write end of stream, check for errors
        if let Err(e) = stream.write(&buffer[..bytes_read]) {
            eprintln!("Failed to write to stream {:?}: {:?}", stream, e);
            return Err(e);
        }
    }
}
