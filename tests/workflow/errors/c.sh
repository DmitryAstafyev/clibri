cd ../../../cli
cargo build --release
cd ../tests/workflow/errors

../../../cli/target/release/clibri -s ../prot/protocol.prot -wf ./c.workflow
