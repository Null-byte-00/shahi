mod core;
mod parse;
mod commands;
mod server;
use std::env;

fn main() {
    let mut port = 7878;
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        port = args[1].parse::<i32>().unwrap();
    }
    server::run_server(port);
}
