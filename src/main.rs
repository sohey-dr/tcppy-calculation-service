mod server;
mod domain;
mod application;
mod adapter;

fn main() -> std::io::Result<()> {
    server::run()
}
