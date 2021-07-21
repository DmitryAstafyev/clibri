#!/bin/sh
cargo build --release
exec 2>&1 # redirect output of stderr to stdout 
ulimit -n 409600
exec ./target/release/server-test