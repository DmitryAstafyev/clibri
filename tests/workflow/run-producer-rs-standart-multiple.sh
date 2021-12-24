#!/bin/sh
reset

cd ./producer/rust
cargo build --release
cd ../..

exec 2>&1 # redirect output of stderr to stdout 
ulimit -n 409600
export CLIBRI_LOG_LEVEL=warn
exec ./producer/rust/target/release/clibri_producer_rs --connections=5000 --multiple
