# Simple Rust Web Server

This project is a simple web server written in Rust. It serves static files from a specified document root and handles basic HTTP requests concurrently. This project is intended as an educational tool and a starting point for building more complex web servers.

## Features

- **Concurrent Request Handling:** Uses threads to serve multiple clients at once.
- **Static File Serving:** Reads and serves files from a user-specified document root.
- **Custom HTTP Error Handling:** Returns appropriate HTTP status codes for bad requests, missing files, and internal server errors.
- **Lightweight & Simple:** Minimal codebase ideal for learning and small projects.

## Installation

To build and run the web server, you need to have [Rust](https://www.rust-lang.org/) installed on your system.

1. Clone the repository:

   ```bash
   git clone git@github.com:Paul-Dejean/web-server.git
   cd web-server
   ```

2. Build the project using Cargo:

   ```bash
   cargo build --release
   ```

## Usage

Run the server:

```bash
sudo cargo run --release
```

The server is bind to the address 127.0.0.1:80 and serve files from the configured folder www.
Type localhost in your browser to see the website served by the server

## Author

Paul Dejean (pauldejeandev@gmail.com)
