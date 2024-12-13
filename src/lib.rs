use std::net::{SocketAddr, UdpSocket};

/// Retrieves the local IP address of the current machine by connecting to a UDP socket.
///
/// # Returns
/// - `Ok(String)`: The local IP address as a string.
/// - `Err(std::io::Error)`: If there is an error binding or connecting the socket.
pub fn get_local_ip() -> Result<String, std::io::Error> {
    const BIND_ADDRESS: &str = "0.0.0.0:0";
    const REMOTE_ADDRESS: &str = "192.168.0.0:0";

    let socket: UdpSocket = UdpSocket::bind(BIND_ADDRESS)?;
    socket.connect(REMOTE_ADDRESS)?;

    let local_addr: SocketAddr = socket.local_addr()?;

    Ok(local_addr.ip().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_ip() {
        let local_ip: String = get_local_ip().unwrap();
        assert!(!local_ip.is_empty());
    }
}
