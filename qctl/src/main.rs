use std::{os::unix::net::UnixStream, process::exit};
use std::io::Write;

fn main() {
    let mut args = std::env::args();

    if args.len() < 3 {
        println!("Not enough arguments were provided.");
        exit(0);
    }

    let sock = UnixStream::connect("/tmp/quasarwm.sock");

    match sock {
        Ok(mut sock) => {
            let args = args.collect::<Vec<String>>();
            println!("{}", args.join(" "));
            let key = args[1].clone();
            let value = args[2].clone();

            sock.write(format!("{} {}", key, value).as_bytes()).unwrap();
        }

        Err(e) => {
            println!("Failed to connect socket: {}", e)
        }
    }
}
