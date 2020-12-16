cd ../../lib-cli
cargo build
cd ../protocol/test
../../lib-cli/target/debug/fiber-cli --src ./prot/protocol.prot -rs ./rust/src/protocol.rs -ts ./typescript/src/protocol.ts -o --em

echo "Builds"
cd ./typescript
npm run build
cd ..

cd ./rust
cargo build
cd ..

echo "Writes"
cd ./typescript
node ./dist/index.js write
cd ..

cd ./rust
./target/debug/fiber_protocol_rust_test write
cd ..

echo "Reads"
cd ./typescript
node ./dist/index.js read
cd ..

cd ./rust
./target/debug/fiber_protocol_rust_test read
cd ..
