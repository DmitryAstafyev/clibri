cd ../../cli
cargo build --release
cd ../tests/workflow

../../cli/target/release/clibri -s ./prot/protocol.prot -wf ./prot/protocol-rs-ts.workflow -cd ./consumer/typescript/src/consumer/ -pd ./producer/rust/src/producer/

cd ./producer/rust
cargo build --release
cd ../..

cd ./consumer/typescript
npm install
npm run build
cd ../..