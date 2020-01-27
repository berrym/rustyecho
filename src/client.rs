use std::{
    error,
    io::{self, BufRead, BufReader, Write},
    net::{SocketAddr, TcpStream},
    str,
    time::Duration,
};

#[macro_use]
extern crate clap;

fn main() -> Result<(), Box<dyn error::Error>> {
    // Parse command line
    let matches = clap_app!(rustyecho =>
        (version: "0.1.1")
        (author: "Michael Berry <trismegustis@gmail.com>")
        (about: "Echo client")
        (@arg ADDR: -a --addr +takes_value +required "Address of server")
        (@arg PORT: -p --port +takes_value +required "Server port")
    )
    .get_matches();

    let addr = matches.value_of("ADDR").unwrap();
    let port = matches.value_of("PORT").unwrap();

    // Set remote address
    let remote: SocketAddr = format!("{}:{}", addr, port).parse()?;

    // Connect to remote with a TCP stream
    println!("Connecting to {:?}", remote);
    let mut stream = TcpStream::connect(&remote)?;
    println!("Successfully connected to remote...");
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;

    // Main loop
    loop {
        let mut input = String::new(); // string to store user input

        // Read a line of input from stdin
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        // Write the input to the TCP stream
        match stream.write(input.as_bytes()) {
            Err(e) => {
                // An error occured, exit program
                eprintln!("{:?}", e);
                std::process::exit(1);
            }
            Ok(_) => {
                // Read line back from remote
                let buffer = read_line_from_remote(&stream)?;
                print!("{}", str::from_utf8(&buffer)?);
            }
        }
    }
}

fn read_line_from_remote(stream: &TcpStream) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let mut buffer: Vec<u8> = Vec::new(); // Buffer to read into
    let mut reader = BufReader::new(stream); // Reader to read from a TCP stream

    // Read one line delimited by the newline char from remote stream
    reader.read_until(b'\n', &mut buffer)?;

    // Return line in buffer
    Ok(buffer)
}
