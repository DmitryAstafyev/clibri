# Logs (rust)
Get rid of next libs everywhere:
- log
- log4rs
Use instead own implementation like in producer/rust
- use only one "error" event instead 3: lib/rust/src/transport/server/events.rs