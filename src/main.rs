mod core;
mod parse;
mod commands;
mod server;

fn main() {
    server::run_server();
}
