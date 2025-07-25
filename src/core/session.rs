use std::{io, sync::mpsc, thread};
use colored::Colorize;

use crate::core::{connect::{call_ping, scan_ports}, utils::build_ip};

pub struct PingSweepSession {
    pattern: String,
    octet_from: u8,
    octet_to: u8,
    deadline: u32,
    enable_port_scan: bool,
    start_port: u16,
    end_port: u16,
    timeout: u64
}

impl PingSweepSession {
    pub fn new(
        pattern: &str, 
        octet_from: u8, 
        octet_to: u8, 
        deadline: u32, 
        enable_port_scan: bool, 
        start_port: u16, 
        end_port: u16,
        timeout: u64
    ) -> Self {
        Self { 
            pattern: String::from(pattern), // Address pattern (e.g. 192.168.172.x)
            octet_from, // Last octet of first host to ping (range start) [default: 1]
            octet_to, // Last octet of last host to ping (range end) [default: 255]
            deadline, // Time to wait for response in seconds [default: 1]
            enable_port_scan, // Enable port scanning for each reachable host
            start_port,
            end_port,
            timeout // Port connect timeout in seconds
        }
    }

    pub fn start_scan(&self) -> io::Result<()> {
        // Ensure correct scan range
        if self.octet_from >= self.octet_to {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput, 
                "Range value `from` cannot be bigger than `to`"
            ));
        }

        let (sender, receiver) = mpsc::channel();
        let deadline = self.deadline;

        for octet in self.octet_from..=self.octet_to {
            if self.pattern.contains(".x") {
                // Construct the target host address
                let address = build_ip(&self.pattern, octet);
                let sender = sender.clone();

                thread::spawn(move || {
                    match call_ping(&address, deadline) {
                        Some(result) => {
                            if result {
                                sender.send(address).unwrap();
                            }
                        },
                        None => {
                            sender.send(String::from("Address verification failed")).unwrap();
                        }
                    }
                });
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid pattern"));
            }
        }

        drop(sender); 

        let mut host_count = 0;
        let mut port_count = 0;

        for result in receiver {
            if result.contains("failed") {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, result));
            }

            host_count += 1;

            let host_ip = result;

            if self.enable_port_scan {
                println!("==================================");
            }
            println!("{} {}{}", "[✔]".green(), host_ip, " ".repeat(10));

            if self.enable_port_scan {
                println!("Scanning for open ports..");

                match scan_ports(&host_ip, self.start_port, self.end_port, self.timeout) {
                    Ok(ports) => {
                        for port in ports {
                            println!("[TCP] {}: {}{}", "Open".green(), port, " ".repeat(10));

                            port_count += 1;
                        }
                    },
                    Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err))
                };
                
            }
        }

        println!("\nEnumeration complete!");
        println!(
            "{} hosts and {} open ports were identified.", 
            host_count, port_count
        );

        Ok(())
    }
}