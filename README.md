# WebVOWL Reimagined

This branch tracks development of WebVOWL Reimagined, which is a total rewrite of WebVOWL in Rust.

The rewrite must satisfy the following:

-   The GUI must be similar to the [original WebVOWL](https://github.com/VisualDataWeb/WebVOWL)

## Run using Docker

Pull image: `docker pull ghcr.io/webvowl/webvowl-reimagined:latest`

Or use the [docker compose file](/docker-compose.yml) with command `docker-compose up -d`

### Building the docker image

0. Make sure Docker is installed
1. Clone the project locally, e.g. `git clone https://github.com/WebVOWL/WebVOWL-Reimagined.git`
2. Make sure you're in the WebVOWL folder, e.g. `cd WebVOWL-Reimagined`
3. To build the docker image run `docker build . -t webvowl-reimagined-dev`
4. To start the docker image run `docker run -d -p 8080 --name webvowl webvowl-reimagined-dev`
5. Visit [http://localhost:8080](http://localhost:8080) to use WebVOWL

## Development setup

[!NOTE]

> Using Linux is recommended

0. Clone the project locally, e.g. `git clone https://github.com/WebVOWL/WebVOWL-Reimagined.git`
1. Install Rust from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Install the lld linker, e.g. `dnf install lld`
3. Run `cargo install leptosfmt`
4. Run `cargo install --locked cargo-leptos`
    > If you get a compile error `Can't locate FindBin.pm in @INC` you can either install Perl (e.g. `dnf install perl`) or [download a prebuilt binary](https://github.com/leptos-rs/cargo-leptos/releases/latest)
5. Use the convenience shell file `build.sh` to build the project with different profiles based on the supplied argument. E.g. to build and run a development server, run `build.sh dev`
