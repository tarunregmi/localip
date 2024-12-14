# localip

A lightweight Rust library for discovering the local IP address of your machine. It uses a simple, connectionless UDP-based approach to reliably determine the local IP address without requiring internet connectivity or third-party dependencies.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
localip = "0.1.0"
```

## Example:

```rust
use localip::get_local_ip;

fn main() {
    match get_local_ip() {
        Ok(ip) => println!("Local IP address: {}", ip),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```
