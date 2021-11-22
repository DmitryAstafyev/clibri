cd ../../cli
cargo build --release
cd ../tests/workflow

../../cli/target/release/clibri -s ./prot/protocol.prot -wf ./prot/protocol-ts-rs.workflow -cd ./consumer/rust/src/consumer/ -pd ./producer/typescript/src/producer/

cd ./producer/typescript
npm install
npm run build
cd ../..

cd ./consumer/rust
cargo build --release
cd ../..