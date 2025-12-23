# Performance Debugging

This is crate runs all code of WebVOWL Reimagined natively to take advantage
of the suite of debugging and performance optimization tools available in Rust.

## Building and Running

To build this crate, open a terminal and run: `cargo build -p perfdebugger --release --target "x86_64-unknown-linux-gnu"`

Note that it does take a while to compile it.

To run the binary, run: `RUST_BACKTRACE=1 ./target/x86_64-unknown-linux-gnu/release/perfdebugger <path/to/file>`
