#!/bin/sh
reset
cargo build --release
exec 2>&1 # redirect output of stderr to stdout 
ulimit -n 409600
export clibri_WS_SERVER_DEBUG_MODE=true
exec ./target/release/server-test