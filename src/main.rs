mod server;
mod domain;

fn main() -> std::io::Result<()> {
    server::run()
}
