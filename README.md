# rustyecho

Echo client and server.

## Description

Small command line utilities to echo text back and forth between clients and server.

## Getting started

Install a recent version of Rust using your OS distributions package manager or Mozilla's own preferred rustup.  For details check with your OS distribution or visit https://rust-lang.org for more information.

### Installing

Clone the git repository from https://github.com/berrym/rustyecho.git

### Building program

Use Rust's own tooling to compile and run the program, e.g.

* cargo build

## Help

Usage can be invoked by passing the --help flag, e.g.

* cargo run --bin echo_server -- --help
* cargo run --bin echo_client -- --help

To quit the program type C^c

## Authors

Copyright 2020
Michael Berry <trismegustis@gmail.com>

## Version History

* 0.1.2
    * Use clap to process command line
* 0.1.1
    * Use getargs to process command line
* 0.1.0
    * Initial Release

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

The excellent and freely available Rust book, for more information visit https://rust-lang.org
