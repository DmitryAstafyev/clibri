cd ../../../lib-cli
cargo build --release
cd ../examples/chat/protocol


if ! ../../../lib-cli/target/release/fiber-cli --src ./protocol.prot -rs ../producer-rust/src/producer/implementation/protocol/mod.rs -o --em; then
    exit 1
fi
if ! ../../../lib-cli/target/debug/fiber-cli --src ./protocol.prot -ts ../consumer-typescript/src/consumer/protocol/protocol.ts -o --em; then
    exit 1
fi
