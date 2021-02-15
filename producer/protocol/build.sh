cd ../../lib-cli
cargo build
cd ../producer/protocol


if ! ../../lib-cli/target/debug/fiber-cli --src ./prot/protocol.prot -rs ../rust/src/protocol/protocol.rs -o --em; then
    exit 1
fi
