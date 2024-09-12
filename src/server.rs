use super::commands;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};



pub fn run_server(port: i32) {
    let mut server = "127.0.0.1:".to_string();
    server.push_str(port.to_string().as_str());
	let listener = TcpListener::bind(server.as_str()).unwrap();
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



