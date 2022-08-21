mod server;
mod domain;
mod application;
mod adpter;

fn main() -> std::io::Result<()> {
    server::run()
}
