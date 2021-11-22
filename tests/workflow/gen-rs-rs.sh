cd ../../cli
cargo build --release
cd ../tests/workflow

../../cli/target/release/clibri -s ./prot/protocol.prot -wf ./prot/protocol-rs-rs.workflow -cd ./consumer/rust/src/consumer/ -pd ./producer/rust/src/producer/

cd ./producer/rust
cargo build --release
cd ../..

cd ./consumer/rust
cargo build --release
cd ../..