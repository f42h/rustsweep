use std::{io, net::TcpStream, time::Duration};

use iptools::ipv4::validate_ip;
use popen_rs::Popen;

use crate::{core::utils::to_sock_addr, correct_range};

pub fn scan_ports(ip: &str, port_from: u16, port_to: u16, timeout: u64) -> Result<Vec<u16>, io::Error> {
    if !correct_range!(port_from, port_to) {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid port range"))
    }

    let mut open_ports: Vec<u16> = Vec::new(); 

    for port in port_from..port_to {
        let address = format!("{}:{}", ip, port);

        // Convert type of address from String to SocketAddr
        to_sock_addr(&address).map(|address_sockaddr| {
            if TcpStream::connect_timeout(&address_sockaddr, Duration::from_millis(timeout)).is_ok() {
                open_ports.push(port);
            }
        })?;
    }

    Ok(open_ports)
}

pub fn call_ping(ip: &str, deadline: u32) -> Option<bool> {
    if !validate_ip(ip) {
        return None;
    }

    // `-c`: Ping count
    // `-w`: Response deadline in seconds
    let command = format!("ping -c 1 {} -w {}", ip, deadline);
    let mut process = Popen::new(&command);

    if let Ok(output) = process.spawn() {
        return Some(output.contains("bytes from"));
    }

    Some(false)
}