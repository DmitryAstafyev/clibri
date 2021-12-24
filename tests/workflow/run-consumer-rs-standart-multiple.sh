#!/bin/sh
reset

cd ./consumer/rust
cargo build --release
cd ../..

exec 2>&1 # redirect output of stderr to stdout 
ulimit -n 409600
export CLIBRI_LOG_LEVEL=warn
exec ./consumer/rust/target/release/clibri_client_rs --connections=5000 --multiple
