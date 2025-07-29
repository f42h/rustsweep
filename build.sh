#!/bin/bash

cargo build --release

BIN="./target/release/rustsweep"

# Enumerate private IPv4 and setup base pattern
IP_ADDRESS=$(hostname -I | awk '{print $1}')
IP_ADDRESS_PATTERN="${IP_ADDRESS%.*}.x"

# Port scan enabled with `-e` flag
START_PORT=1
LAST_PORT=1000

# TCP connect timeout in milliseconds
TIMEOUT=5

### Examples:

function full_scan() {
    echo "Starting full scan.."
    sudo $BIN -p $IP_ADDRESS_PATTERN -e -f $START_PORT -l $LAST_PORT -t $TIMEOUT -i --http-test
}

function scan_port_knocking() {
    echo "Starting host enumeration and check for open ports.."
    $BIN -p $IP_ADDRESS_PATTERN -e -f $START_PORT -l $LAST_PORT
}

function simple_enumeration() {
    echo "Pingsweeping the current network.."
    $BIN -p $IP_ADDRESS_PATTERN
}

printf "(%s) " $0

# Do a full scan: Enumerate hosts, scan for open ports (1-1000) and check for web ui/server
full_scan

# Pingsweep the current network and check for open ports (1-1000) on each host found
#scan_port_knocking

# Simple host enumeration (only using ping)
#simple_enumeration