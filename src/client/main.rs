extern crate byteorder;

use byteorder::{ReadBytesExt, WriteBytesExt, NativeEndian};

use std::net;
use std::io;

use io::{BufReader, BufRead};

fn main() {
    let mut stream = match net::TcpStream::connect("127.0.0.1:666") {
        Ok(stream) => stream,
        Err(error) => {
            println!("Failed to connect to server... :(. Error: {}", error);
            return;
        }
    };

    loop {
        let mut line = String::new();
        println!("Onii-chan, please enter number:");

        io::stdin().read_line(&mut line).expect("Couldn't read line!");

        let req_num = match line.trim_right().parse::<u64>() {
            Ok(num) => {
                line.clear();
                num
            }
            Err(_) => {
                println!("Input is not an positive integer! Baka... Try again...");
                continue;
            }
        };

        if req_num == 0 {
            println!("Exit is requested. Bye bye, onii-chan!");
            return;
        }

        match stream.write_u64::<NativeEndian>(req_num) {
            Ok(_) => (),
            Err(error) => {
                println!("Failed to send request. Error: {}", error);
                continue;
            }
        }

        let res_num = match stream.read_u64::<NativeEndian>() {
            Ok(num) => num,
            Err(error) => {
                println!("Failed to read response. Error: {}", error);
                continue;
            }
        };

        println!("Server respond with {} addresses", res_num);

        let mut reader = BufReader::new(&mut stream);
        let _ = reader.read_line(&mut line);
        line.clear();

        for _ in 0..res_num {
            match reader.read_line(&mut line) {
                Ok(_) => println!("IP:{}", line.trim_right()),
                Err(error) => println!("Error reading response: {}", error),
            }
            line.clear()
        }

        println!("");
    }
}
