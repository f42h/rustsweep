use std::{
    fs::File, 
    io::{self, BufRead}, 
    path::Path, 
    process::{exit, Command}
};
use iptools::ipv4::validate_ip;

fn get_organization(mac: &str, db: &str) -> Option<String> {
    // Path to the OUI data file
    let mut oui_db = "oui.txt"; 

    if db.is_empty() && !Path::new(oui_db).exists() {
        eprintln!("Error: {} does not exist but is required for the OUI lookup feature", oui_db);
        eprintln!("Please specify the location using `-o`");
        exit(1);
    }

    if !db.is_empty() {
        oui_db = db;
    }

    // Open db
    match File::open(oui_db) {
        Ok(stream) => {
            let reader = io::BufReader::new(stream);

            for line in reader.lines() {
                let entry = line.unwrap();
                let parts: Vec<&str> = entry.split(",").collect();

                if parts.len() == 2 {
                    let oui = parts[0].trim();
                    let organization = parts[1].trim();

                    if mac.starts_with(oui) {
                        return Some(organization.to_string());
                    }
                }
            }
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            return None;
        }
    };

    None
}

// Lookup OUI based on MAC address
pub fn oui_lookup(mac: &str, db: &str) -> Option<String> {
    let normalized_mac = mac.to_uppercase().replace(&[':', '-'][..], "");
    
    if normalized_mac.chars().all(|c| c.is_ascii_hexdigit()) {
        return get_organization(&normalized_mac, db);
    }

    None
}

pub fn request_mac(ip: &str) -> Option<String> {
    if !validate_ip(ip) {
        return None;
    }

    if let Ok(output) = Command::new("arp")
        .arg("-a")
        .arg(ip)
        .output() {
        let re = regex::Regex::new(r"([0-9a-fA-F]{1,2}:){5}[0-9a-fA-F]{1,2}").unwrap();
        for cap in re.captures_iter(&String::from_utf8_lossy(&output.stdout)) {
            return Some(cap[0].to_string())
        }
    }

    None
}