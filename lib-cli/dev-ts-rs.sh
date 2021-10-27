cargo build
reset
./target/debug/fiber-cli -s ../environment/protocol/prot/protocol.prot -rs ../environment/protocol/prot/protocol.rs -o -wf ../environment/protocol/prot/protocol.workflow -cd ./tmp/client/src/consumer/ -pd ./tmp/server/src/producer/