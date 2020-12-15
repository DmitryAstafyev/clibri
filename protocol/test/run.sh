cd ../../lib-cli
cargo build
cd ../protocol/test
../../lib-cli/target/debug/fiber-cli --src ./prot/protocol.prot -rs ./rust/src/protocol.rs -ts ./typescript/src/protocol.ts -o --em
cd ./typescript
npm run build
node ./dist/index.js
cd ..
cd ./rust
cargo run
cd ..
