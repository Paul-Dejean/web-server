use web_server::launch_server;

fn main() -> std::io::Result<()> {
    launch_server("127.0.0.1:80")
}
