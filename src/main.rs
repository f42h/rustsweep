mod core;

use crate::core::{session::PingSweepSession, utils::is_root};
use crate::core::utils::build_ip;

use std::{process::exit, time::Instant};
use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pattern: String,

    #[arg(short, long, default_value_t = 1)]
    start_from: u8,

    #[arg(short, long, default_value_t = 255)]
    range_to:u8,

    #[arg(short, long, default_value_t = 1)]
    deadline: u32,

    #[arg(short, long, action)]
    enable_port_scan: bool,

    #[arg(short, long, default_value_t = 1)]
    first_port: u16,

    #[arg(short, long, default_value_t = 65535)]
    last_port: u16,

    #[arg(short, long, default_value_t = 10)]
    timeout: u64,

    #[arg(short, long, action)]
    inspect: bool,

    #[arg(short, long, default_value="")]  
    oui_db_path: String
}

fn worker() {
    let args = Args::parse();
    let session = PingSweepSession::new(
        &args.pattern,
        args.start_from,
        args.range_to,
        args.deadline,
        args.enable_port_scan,
        args.first_port,
        args.last_port,
        args.timeout,
        args.inspect,
        &args.oui_db_path
    );

    let start_host = build_ip(&args.pattern, args.start_from);

    println!("Range      : {}-{}", start_host, args.range_to);
    println!("Deadline   : {}s", args.deadline);

    if args.enable_port_scan {
        println!("Portscan   : Enabled");
        println!("Start port : {}", args.first_port);
        println!("Last port  : {}", args.last_port);
    } else {
        println!("Portscan   : Disabled");
        println!("==================================");
    }

    match is_root() {
        Some(result) => {
            if !result && args.inspect {
                eprintln!("Root is required for `-i`!");
                return;
            }
        },
        None => {
            eprintln!("Failed to get UID!");
            return;
        },
    }
    
    session.start_scan().map_err(|err| {
        eprintln!("{} Error: {}", "[✘]".red(), err);
        exit(1);
    }).unwrap();
}

fn main() {
    let start_time = Instant::now();

    {
        println!("\n{}", "            Rustsweep".italic().cyan());
        println!("{}",   "    Fast CLI Pingsweeping Tool   ".bold());
        println!("==================================");

        worker();
    }

    let end_time = Instant::now();
    println!("Time needed: {:#.2?}", end_time - start_time)
}