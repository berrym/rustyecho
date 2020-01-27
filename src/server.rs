use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

#[macro_use]
extern crate clap;

fn main() -> io::Result<()> {
    // Parse command line
    let matches = clap_app!(rustyecho =>
        (version: "0.1.1")
        (author: "Michael Berry <trismegustis@gmail.com>")
        (about: "Echo server")
        (@arg ADDR: -a --addr +takes_value +required "Address to bind to")
        (@arg PORT: -p --port +takes_value +required "Port number to bind to")
    )
    .get_matches();

    let addr = matches.value_of("ADDR").unwrap();
    let port = matches.value_of("PORT").unwrap();

    // Bind to address
    println!("Binding to {}:{}", addr, port);
    let listener = TcpListener::bind(format!("{}:{}", addr, port))?;
    println!("Successfully bound to {}:{}, listening...", addr, port);

    // Main loop
    for stream in listener.incoming() {
        thread::spawn(move || handle_stream(stream?));
    }

    Ok(())
}

fn handle_stream(mut stream: TcpStream) -> io::Result<()> {
    // Buffer to store remote input
    let mut buffer = [0; 512];

    println!("New connection: {:?}", stream);

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
