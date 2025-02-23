use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
    thread,
    time::Duration,
};

use web_server::Server;

#[test]
fn should_respond_with_requested_file() {
    let addr = "127.0.0.1:8000";
    let path = "/index.html";
    thread::spawn(move || {
        let server = Arc::new(Server::new(addr, "www"));
        server.run()
    });
    thread::sleep(Duration::from_millis(200));
    let response = reqwest::blocking::get(format!("http://{}{}", addr, path))
        .expect("Failed to connect to server");
    let body = response.text().expect("Failed to read response body");

    let expected_content = std::fs::read_to_string("www/index.html").expect("Failed to read file");

    assert!(
        body.contains(&expected_content),
        "Unexpected body: {}",
        body
    );
}

#[test]
fn should_respond_with_default_file() {
    let addr = "127.0.0.1:8000";
    let path = "/";
    thread::spawn(move || {
        let server = Arc::new(Server::new(addr, "www"));
        server.run()
    });
    thread::sleep(Duration::from_millis(200));
    let response = reqwest::blocking::get(format!("http://{}{}", addr, path))
        .expect("Failed to connect to server");
    let body = response.text().expect("Failed to read response body");

    let expected_content = std::fs::read_to_string("www/index.html").expect("Failed to read file");

    assert!(
        body.contains(&expected_content),
        "Unexpected body: {}",
        body
    );
}

#[test]
fn should_respond_with_not_found() {
    let addr = "127.0.0.1:8000"; // Use a different port if needed
    let path = "/nonexistent.html";
    thread::spawn(move || {
        let server = Arc::new(Server::new(addr, "www"));
        server.run()
    });
    thread::sleep(Duration::from_millis(200));
    let response = reqwest::blocking::get(format!("http://{}{}", addr, path))
        .expect("Failed to connect to server");

    assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
    let body = response.text().expect("Failed to read response body");

    assert!(
        body.contains("Page Not Found"),
        "Unexpected body for error: {}",
        body
    );
}

#[test]
fn should_not_allow_to_navigate_outside_of_document_root() {
    let addr = "127.0.0.1:8000";
    let path = "/../index.html";
    thread::spawn(move || {
        let server = Arc::new(Server::new(addr, "www"));
        server.run()
    });
    thread::sleep(Duration::from_millis(200));

    let mut stream = TcpStream::connect(addr).expect("Failed to connect to server");
    let request = format!("GET {} HTTP/1.1\r\n", path);
    stream
        .write_all(request.as_bytes())
        .expect("Failed to write to stream");

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .expect("Failed to read response");

    assert!(
        response.contains("400") || response.contains("Bad Request"),
        "Expected 400 Bad Request, got: {}",
        response
    );
}
