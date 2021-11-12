cd ../lib-cli
cargo build --release
cd ../examples

../lib-cli/target/release/fiber-cli -s ./prot/protocol.prot -wf ./prot/protocol-rs-ts.workflow -cd ./consumer/typescript/src/consumer/ -pd ./producer/rust/src/producer/

cd ./producer/rust
cargo build --release
cd ../..

cd ./consumer/typescript
npm install
npm run build
cd ../..