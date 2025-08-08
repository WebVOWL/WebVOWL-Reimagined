# WebVOWL Rust
This branch tracks development of WebVOWL Rust, which is a total rewrite of WebVOWL in Rust. 

The rewrite must satisfy the following:
- The GUI must be similar to the [original WebVOWL](https://github.com/VisualDataWeb/WebVOWL)

## Run using Docker
TBD

## Development setup
1. Install Maven from https://maven.apache.org/download.cgi
2. Install Rust from https://www.rust-lang.org/tools/install
3. Run `cargo install wasm-pack`

Next, make sure you are inside the `WebVOWL` directory.
Now you can:
- Run `mvn package` to build the project
- Run `mvn package -P dev-server` to start a local webserver with the current development version
