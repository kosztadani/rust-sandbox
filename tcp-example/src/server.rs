mod common;

use common::MathRequest;
use common::MathOperation;
use std::convert::TryInto;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::process::exit;
use std::{io, thread};

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
            // get real port number if 0 was passed in
            let listen_port = listener.local_addr().unwrap().port();
            println!("Listening on port {}", listen_port);
            listener
        }
        Err(_) => {
            eprintln!("Error: could not listen on port {}", port);
            exit(1);
        }
    };
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    let mut connection = MathServerConnection::new(&stream);
                    connection.handle();
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

#[derive(Debug)]
struct MathServerConnection<'a> {
    reader: BufReader<&'a TcpStream>,
    writer: BufWriter<&'a TcpStream>,
    buffer: [u8; 9],
}

impl<'a> MathServerConnection<'a> {
    fn new(stream: &'a TcpStream) -> Self {
        MathServerConnection {
            reader: BufReader::new(stream),
            writer: BufWriter::new(stream),
            buffer: [0; 9],
        }
    }

    fn handle(&mut self) {
        loop {
            match self.process() {
                Ok(_) => continue,
                Err(_) => {
                    // client probably disconnected
                    break;
                }
            }
        }
    }

    fn process(&mut self) -> Result<(), io::Error> {
        self.fill_buffer()?;
        let request = self.parse_request();
        // intentional "bug": divide by zero to crash the thread ;)
        let result = request
            .operator
            .apply(request.first_operand, request.second_operand);
        let result_bytes = result.to_be_bytes();
        self.writer.write_all(&result_bytes)?;
        self.writer.flush()?;
        Ok(())
    }

    fn fill_buffer(&mut self) -> Result<(), io::Error> {
        self.reader.read_exact(&mut self.buffer)
    }

    fn parse_request(&mut self) -> MathRequest {
        let opcode = self.buffer[0];
        let operator = MathOperation::from_opcode(opcode).unwrap();
        let first_operand = i32::from_be_bytes(self.buffer[1..5].try_into().unwrap());
        let second_operand = i32::from_be_bytes(self.buffer[5..9].try_into().unwrap());
        MathRequest {
            operator,
            first_operand,
            second_operand,
        }
    }
}

fn print_help_and_exit() -> ! {
    println!("Usage: server <port>");
    exit(1);
}
