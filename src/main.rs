mod server;
mod domain;
mod application;

fn main() -> std::io::Result<()> {
    server::run()
}
