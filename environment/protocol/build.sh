cd ../../lib-cli
cargo build
cd ../environment/protocol


if ! ../../lib-cli/target/debug/fiber-cli --src ./prot/protocol.prot -rs ../producer/rust/producer/src/protocol/protocol.rs -o --em; then
    exit 1
fi
if ! ../../lib-cli/target/debug/fiber-cli --src ./prot/protocol.prot -ts ../consumer/typescript/src/consumer/protocol/protocol.ts -o --em; then
    exit 1
fi
