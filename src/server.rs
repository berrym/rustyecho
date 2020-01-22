use std::{
    env,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    process, thread,
    time::Duration,
};

use rustyecho::parse_args;

use getargs::Options;

fn main() -> io::Result<()> {
    // Process cli arguments
    let args: Vec<_> = env::args().skip(1).collect();
    let opts = Options::new(&args);
    let options = match parse_args(&opts) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("usage error: {}", e);
            process::exit(1);
        }
    };

    // Bind to address
    println!("Binding to {}:{}", options.address, options.port);
    let listener = TcpListener::bind(format!("{}:{}", options.address, options.port))?;
    println!(
        "Successfully bound to {}:{}, listening...",
        options.address, options.port
    );

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
