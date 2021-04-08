use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::{env, process};
// use std::time::Duration;
// use std::net::SocketAddr;

struct Config {
    ip: String,
    port: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let ip = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get an IP address"),
        };
        let port = match args.next() {
            Some(arg) => arg,
            None => String::from("3333"),
        };
        Ok(Config {
            ip,
            port,
        })
    }
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let server_details = format!("{}:{}", config.ip, config.port);

    // let server: SocketAddr = server_details
    //     .parse()
    //     .expect("Unable to parse socket address");

    // match TcpStream::connect_timeout(&server , Duration::from_secs(5)) {
    match TcpStream::connect(server_details) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}