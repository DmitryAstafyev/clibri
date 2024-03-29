cd ../../cli
cargo build --release
cd ../tests/protocol


if ! ../../cli/target/release/clibri --src ./prot/protocol.prot -rs ./rust/src/protocol.rs -ts ./typescript/src/protocol.ts -o --em; then
    exit 1
fi

echo "Builds"
cd ./typescript
if ! npm run build; then
    exit 1
fi

cd ..

cd ./rust
if ! cargo build --release; then
    exit 1
fi

cd ..

echo "Writes"
cd ./typescript
if ! node ./dist/index.js write; then
    exit 1
fi
cd ..

cd ./rust
if ! ./target/release/clibri_protocol_rust_test write; then
    exit 1
fi
cd ..

echo "Reads"
cd ./typescript
if ! node ./dist/index.js read; then
    exit 1
fi
cd ..

cd ./rust
if ! ./target/release/clibri_protocol_rust_test read; then
    exit 1
fi
cd ..
