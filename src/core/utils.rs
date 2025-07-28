use std::{io, net::SocketAddr, str::FromStr};

use popen_rs::Popen;


pub fn to_sock_addr(address: &str) -> Result<SocketAddr, io::Error> {
    SocketAddr::from_str(address)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))
}

pub fn build_ip(pattern: &str, octet:u8) -> String {
    // Replace placeholder of ip address pattern
    pattern.replace("x", &format!("{}", octet))
}

pub fn is_root() -> Option<bool> {
    if let Ok(output) = Popen::new("id -u").spawn() {
        return Some(output.replace("\n", "") == "0");
    }

    None
}