FROM rustlang/rust:nightly-alpine AS builder

RUN apk update && apk upgrade --no-cache && apk add --no-cache \
    bash \
    curl \
    git \
    binaryen \
    clang \
    lld

# Get script to patch and install mimalloc
# RUN git clone "https://github.com/WebVOWL/rust-alpine-mimalloc"

# WORKDIR /rust-alpine-mimalloc

# Use latest stable version of mimalloc (2025-06-09)
# RUN /rust-alpine-mimalloc/build.sh 2.2.4 SECURE


# Install a prebuilt binary of cargo-leptos
RUN curl --proto '=https' --tlsv1.3 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

WORKDIR /work
COPY . .

# Set LD_PRELOAD to use mimalloc globally
# ENV LD_PRELOAD=/usr/lib/libmimalloc.so

# Override bin-target-triple defined in Cargo.toml
ENV LEPTOS_BIN_TARGET_TRIPLE="x86_64-unknown-linux-musl"

# Build WebVOWL
RUN cargo leptos build --release --precompress -vv


FROM scratch AS runner

WORKDIR /app

USER 10001

COPY --chown=10001 --from=builder /work/target/x86_64-unknown-linux-musl/release/webvowl-reimagined /app/
COPY --chown=10001 --from=builder /work/target/site /app/site

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site

# Depends on the port you choose
EXPOSE 8080

# Must match your final server executable name
CMD ["/app/webvowl-reimagined"]