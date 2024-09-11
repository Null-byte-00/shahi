use super::commands;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};



pub fn run_server() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	let mut storage = commands::Storage::new();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let buf_reader = BufReader::new(&mut stream);
    	let request: Vec<_> = buf_reader
        	.lines()
        	.map(|result| result.unwrap())
        	.take_while(|line| !line.is_empty())
        	.collect();
		let mut output = storage.run_command(request.get(0).unwrap().clone()).unwrap();
		stream.write_all(output.as_bytes()).unwrap();
	}
}



