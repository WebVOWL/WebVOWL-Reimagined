FROM rustlang/rust:nightly-alpine AS builder

RUN apk update && apk upgrade --no-cache && apk add --no-cache \
    bash \
    curl \
    git \
    binaryen \
    clang \
    lld

# # Get script to patch and install mimalloc
# RUN git clone "https://github.com/WebVOWL/rust-alpine-mimalloc"

WORKDIR /rust-alpine-mimalloc

COPY test.sh .

# # Use latest stable version of mimalloc (2025-06-09)
RUN /rust-alpine-mimalloc/test.sh 2.2.4 SECURE

# Install a prebuilt binary of cargo-leptos
RUN curl --proto '=https' --tlsv1.3 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

WORKDIR /work
COPY . .

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

# Testing
ENV MIMALLOC_VERBOSE=1

# The delay in N milli-seconds (by default 10) after which mimalloc will purge OS pages that are not in use.
# Setting N to a higher value like 100 can improve performance (sometimes by a lot) at the cost of potentially
# using more memory at times.
ENV MIMALLOC_PURGE_DELAY=50


# Depends on the port you choose
EXPOSE 8080

# Must match your final server executable name
CMD ["/app/webvowl-reimagined"]