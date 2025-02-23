use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

#[derive(Debug)]
enum HttpError {
    BadRequest(String),
    NotFound(String),
    InternalServerError(String),
}

impl From<std::io::Error> for HttpError {
    fn from(e: std::io::Error) -> Self {
        HttpError::BadRequest(e.to_string())
    }
}

pub struct Server {
    bind_address: String,
    document_root: String,
}

impl Server {
    pub fn new(bind_address: &str, document_root: &str) -> Self {
        Self {
            bind_address: bind_address.to_string(),
            document_root: document_root.to_string(),
        }
    }

    pub fn run(self: Arc<Self>) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.bind_address)?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let server = Arc::clone(&self);
                    thread::spawn(move || {
                        if let Err(error) = server.handle_client(stream) {
                            eprintln!("Client handling failed: {}", error);
                        }
                    });
                }
                Err(error) => eprintln!("Error: {}", error),
            }
        }
        Ok(())
    }

    fn parse_path(&self, path: &str) -> Result<String, &'static str> {
        if path.contains("..") {
            return Err("Invalid path");
        }

        if path == "/" {
            return Ok(format!("{}/index.html", self.document_root));
        }

        Ok(format!("{}/{}", self.document_root, &path[1..]))
    }

    fn send_error_response(stream: &mut TcpStream, error: &HttpError) -> std::io::Result<()> {
        let (status_line, message) = match error {
            HttpError::BadRequest(msg) => ("HTTP/1.1 400 Bad Request", msg),
            HttpError::NotFound(msg) => ("HTTP/1.1 404 Not Found", msg),
            HttpError::InternalServerError(msg) => ("HTTP/1.1 500 Internal Server Error", msg),
        };

        let response = format!("{}\r\n\r\n{}", status_line, message);

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write error response: {}", e);
            return Err(e);
        }

        if let Err(e) = stream.flush() {
            eprintln!("Failed to flush error response: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Internal Server Error (flush failed)",
            ));
        }

        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) -> std::io::Result<()> {
        let result = self.process_request(&mut stream);

        if let Err(err) = result {
            // Attempt to send the error response. We ignore errors from send_error_response here,
            // but you might choose to log them.
            let _ = Self::send_error_response(&mut stream, &err);
            return match err {
                HttpError::NotFound(msg) => Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Not Found: {}", msg),
                )),
                HttpError::BadRequest(msg) => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Bad Request: {}", msg),
                )),
                HttpError::InternalServerError(msg) => Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Internal Server Error: {}", msg),
                )),
            };
        }
        Ok(())
    }

    fn parse_header(stream: &mut TcpStream) -> Result<[String; 3], HttpError> {
        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .map_err(|e| HttpError::BadRequest(e.to_string()))?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.as_slice() {
            [method, path, protocol] => {
                return Ok([method.to_string(), path.to_string(), protocol.to_string()])
            }
            _ => return Err(HttpError::BadRequest("Invalid request".into())),
        }
    }

    fn process_request(&self, stream: &mut TcpStream) -> Result<(), HttpError> {
        let [_, path, _] = Self::parse_header(stream)?;
        let file_path = self
            .parse_path(&path)
            .map_err(|e| HttpError::BadRequest(e.to_string()))?;

        let contents = fs::read(file_path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                HttpError::NotFound("Page Not Found".into())
            } else {
                HttpError::InternalServerError(e.to_string())
            }
        })?;

        let response_header = "HTTP/1.1 200 OK\r\n\r\n";

        stream
            .write_all(response_header.as_bytes())
            .map_err(|e| HttpError::BadRequest(e.to_string()))?;
        stream
            .write_all(&contents)
            .map_err(|e| HttpError::BadRequest(e.to_string()))?;

        stream
            .flush()
            .map_err(|e| HttpError::InternalServerError(e.to_string()))?;

        Ok(())
    }
}
