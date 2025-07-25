use std::{io, net::SocketAddr, str::FromStr};


pub fn to_sock_addr(address: &str) -> Result<SocketAddr, io::Error> {
    SocketAddr::from_str(address)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))
}

pub fn build_ip(pattern: &str, octet:u8) -> String {
    // Replace placeholder of ip address pattern
    pattern.replace("x", &format!("{}", octet))
}