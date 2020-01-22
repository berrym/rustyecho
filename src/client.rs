use std::{
    env, error,
    io::{self, BufRead, BufReader, Write},
    net::{SocketAddr, TcpStream},
    process, str,
    time::Duration,
};

use rustyecho::parse_args;

use getargs::Options;

fn main() -> Result<(), Box<dyn error::Error>> {
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

    // Set remote address
    let remote: SocketAddr = format!("{}:{}", options.address, options.port).parse()?;

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
