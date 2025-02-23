use std::sync::Arc;

use web_server::Server;

fn main() -> std::io::Result<()> {
    let server = Arc::new(Server::new("127.0.0.1:80", "www"));
    server.run()
}
