use std::{thread, time::Duration};
use web_server::launch_server;

#[test]
fn should_respond_with_requested_file() {
    let addr = "127.0.0.1:8000";
    let path = "/index.html";
    thread::spawn(move || {
        launch_server(addr).unwrap();
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
        launch_server(addr).unwrap();
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
        launch_server(addr).unwrap();
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
