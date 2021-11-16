cd ../cli
cargo build --release
cd ../examples

../cli/target/release/fiber -s ./prot/protocol.prot -wf ./prot/protocol-ts-ts.workflow -cd ./consumer/typescript/src/consumer/ -pd ./producer/typescript/src/producer/

cd ./producer/typescript
npm install
npm run build
cd ../..

cd ./consumer/typescript
npm install
npm run build
cd ../..