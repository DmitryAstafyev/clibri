cd ../cli
cargo build --release
cd ../examples

../cli/target/release/clibri -s ./prot/protocol.prot -wf ./prot/protocol-rs-rs.workflow --puml ./plantuml/plantuml.puml
