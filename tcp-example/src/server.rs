mod common;

use common::MathOperation;
use std::convert::TryInto;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::process::exit;
use std::thread;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        print_help_and_exit();
    } else {
        match args.get(1) {
            Some(port_str) => match port_str.parse::<u16>() {
                Ok(port) => start_server(port),
                Err(_) => print_help_and_exit(),
            },
            None => print_help_and_exit(),
        }
    }
}

fn start_server(port: u16) {
    let address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let listener = match TcpListener::bind(address) {
        Ok(listener) => {
            println!("Listening on port {}", port);
            listener
        }
        Err(_) => {
            println!("Error: could not listen on port {}", port);
            exit(1);
        }
    };
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let mut buffer = [0; 9];
    loop {
        match reader.read_exact(&mut buffer) {
            Ok(_) => {
                let opcode = buffer[0];
                let operation = MathOperation::from_opcode(opcode);
                let left = i32::from_be_bytes(buffer[1..5].try_into().unwrap());
                let right = i32::from_be_bytes(buffer[5..9].try_into().unwrap());
                // intentional "bug": divide by zero to crash the thread ;)
                let result = operation.unwrap().apply(left, right);
                let result_bytes = result.to_be_bytes();
                let _ = writer.write_all(&result_bytes);
                let _ = writer.flush();
            }
            Err(_) => {
                // client probably disconnected
            }
        }
    }
}

fn print_help_and_exit() {
    println!("Usage: server <port>");
    exit(1);
}
