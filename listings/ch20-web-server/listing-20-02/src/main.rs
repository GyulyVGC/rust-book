use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const SOCKET: &str = "192.168.1.251:999";

fn main() {
    let listener = TcpListener::bind(SOCKET).unwrap();
    println!("\nServer is up and listening at {}\n", SOCKET);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("New request: {:#?}\n", http_request);
}
