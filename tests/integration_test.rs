use std::{thread, time::Duration};
use web_server::launch_server;

#[test]
fn should_respond_with_requested_path() {
    let addr = "127.0.0.1:8000";
    let path = "/hello";
    thread::spawn(move || {
        launch_server(addr).unwrap();
    });
    thread::sleep(Duration::from_millis(200));
    let response = reqwest::blocking::get(format!("http://{}{}", addr, path))
        .expect("Failed to connect to server");
    let body = response.text().expect("Failed to read response body");
    assert!(
        body.contains(format!("Requested path: {}", path).as_str()),
        "Unexpected body: {}",
        body
    );
}
