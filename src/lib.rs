use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

pub fn launch_server(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(error) = handle_client(stream) {
                    eprintln!("Client handling failed: {}", error);
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(&stream);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let parts = line.split_whitespace().collect::<Vec<_>>();
    if let [_, path, _] = parts.as_slice() {
        let response = format!("HTTP/1.1 200 OK\r\n\r\nRequested path: {}\r\n", path);
        stream.write_all(response.as_bytes())?;
        stream.flush()?;

        Ok(())
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid request",
        ));
    }
}
