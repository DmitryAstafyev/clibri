cd ../../../lib-cli
cargo build --release
cd ../examples/chat


if ! ../../lib-cli/target/release/fiber-cli --src ./protocol/protocol.prot -rs ./producer-rust/src/protocol/mod.rs -o --em; then
    exit 1
fi
#if ! ../../lib-cli/target/debug/fiber-cli --src ./protocol.prot -ts ../consumer-ts/src/consumer/protocol/protocol.ts -o --em; then
#    exit 1
#fi
