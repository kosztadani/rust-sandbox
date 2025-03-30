mod common;

use common::MathOperation;
use common::MathRequest;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::process::exit;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        // e.g., "localhost:9999"
        let server = args.get(1).unwrap();
        run(server);
    } else if args.len() == 3 {
        // e.g., "localhost" and "9999"
        let host = args.get(1).unwrap();
        let port = match args.get(2).unwrap().parse::<u16>() {
            Ok(value) => value,
            Err(_) => print_help_and_exit(),
        };
        run((host.as_str(), port));
    } else {
        print_help_and_exit();
    }
}

fn run<A: ToSocketAddrs>(address_provider: A) {
    match TcpStream::connect(address_provider) {
        Ok(stream) => {
            let client = MathClient::new(&stream);
            let mut cli = MathCli::new(client);
            cli.run();
        }
        Err(_) => {
            eprintln!("Error: couldn't connect to server.");
            exit(1);
        }
    }
}

#[derive(Debug)]
enum Error {
    FatalError,
    InvalidRequest,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FatalError => f.write_str("Fatal error"),
            Self::InvalidRequest => f.write_str("Invalid request"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(_value: io::Error) -> Self {
        Self::FatalError
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
struct MathCli<'a> {
    client: MathClient<'a>,
    input_buffer: String,
}

impl<'a> MathCli<'a> {
    fn new(client: MathClient<'a>) -> Self {
        Self {
            client,
            input_buffer: String::new(),
        }
    }

    fn run(&mut self) {
        println!("Input operations, e.g., \"5 + 3\":");
        loop {
            match self.prompt_and_execute() {
                Ok(_) => continue,
                Err(e) => match e {
                    Error::FatalError => {
                        Self::handle_fatal_error(e);
                    }
                    Error::InvalidRequest => {
                        continue;
                    }
                },
            }
        }
    }

    fn prompt_and_execute(&mut self) -> Result<(), Error> {
        let request = self.prompt_user()?;
        let result = self.client.request(request)?;
        println!("{}", result);
        Ok(())
    }

    fn prompt_user(&mut self) -> Result<MathRequest, Error> {
        print!(">>> ");
        io::stdout().flush()?;
        self.input_buffer.clear();
        io::stdin().read_line(&mut self.input_buffer)?;
        self.parse_input()
    }

    fn parse_input(&mut self) -> Result<MathRequest, Error> {
        let mut tokens = self.input_buffer.split_whitespace();
        let request = MathRequest {
            first_operand: MathCli::parse_operand(tokens.next())?,
            operator: MathCli::parse_operator(tokens.next())?,
            second_operand: MathCli::parse_operand(tokens.next())?,
        };
        Ok(request)
    }

    fn parse_operand(input: Option<&str>) -> Result<i32, Error> {
        input
            .ok_or(Error::InvalidRequest)
            .and_then(|s| i32::from_str(s).map_err(|_| Error::InvalidRequest))
    }

    fn parse_operator(input: Option<&str>) -> Result<MathOperation, Error> {
        input
            .and_then(MathOperation::from_str)
            .ok_or(Error::InvalidRequest)
    }

    fn handle_fatal_error(e: Error) -> ! {
        eprintln!("Error: {}", e);
        exit(1);
    }
}

#[derive(Debug)]
struct MathClient<'a> {
    reader: BufReader<&'a TcpStream>,
    writer: BufWriter<&'a TcpStream>,
    response_buffer: [u8; 4],
}

impl<'a> MathClient<'a> {
    fn new(stream: &'a TcpStream) -> Self {
        Self {
            reader: BufReader::new(stream),
            writer: BufWriter::new(stream),
            response_buffer: [0; 4],
        }
    }

    fn request(&mut self, request: MathRequest) -> Result<i32, Error> {
        self.writer.write_all(&request.to_bytes())?;
        self.writer.flush()?;
        self.reader.read_exact(&mut self.response_buffer)?;
        Ok(i32::from_be_bytes(self.response_buffer))
    }
}

fn print_help_and_exit() -> ! {
    println!("Usage: client <address> <port>");
    println!("       client <address>:<port>");
    exit(1);
}
