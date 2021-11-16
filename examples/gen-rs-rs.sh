cd ../cli
cargo build --release
cd ../examples

../cli/target/release/fiber -s ./prot/protocol.prot -wf ./prot/protocol-rs-rs.workflow -cd ./consumer/rust/src/consumer/ -pd ./producer/rust/src/producer/

cd ./producer/rust
cargo build --release
cd ../..

cd ./consumer/rust
cargo build --release
cd ../..