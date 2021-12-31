cd ../../cli
cargo build --release
cd ../examples/protocol


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
