use reqwest::{header::HeaderName, StatusCode};

pub struct UrlBuild {
    protocol: String,
    hostname: String,
    port: u16
}

impl UrlBuild {
    pub fn new(protocol: &str, hostname: &str, port: u16) -> Self {
        Self { 
            protocol: String::from(protocol), 
            hostname: String::from(hostname), 
            port 
        }
    }

    pub fn construct(&self) -> String {
        if self.port != 0 {
            format!("{}://{}:{}", self.protocol, self.hostname, self.port)
        } else {
            format!("{}://{}", self.protocol, self.hostname)
        }
    }
}

pub fn http_request(url: &str) -> Result<StatusCode, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let status_code = response.status();

    if status_code.is_success() {
        println!("  {:<20} : {}", "status code", status_code);

        if let Some(value) = response
            .headers()
            .get(HeaderName::from_lowercase("server".as_bytes()).unwrap()) {
            println!("  {:<20} : {}", "server", value.to_str().unwrap());
        } 
    }
    
    Ok(status_code)
}