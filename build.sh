#!/bin/bash

cargo build --release

BIN="./target/release/rustsweep"

IP_ADDRESS_PATTERN="192.168.99.x"

# Port scan enabled with `-e` flag
START_PORT=1
LAST_PORT=500

# TCP connect timeout in milliseconds
TIMEOUT=5

$BIN -p $IP_ADDRESS_PATTERN -e -f $START_PORT -l $LAST_PORT -t $TIMEOUT