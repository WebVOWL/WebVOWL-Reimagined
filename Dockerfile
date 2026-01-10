FROM rustlang/rust:nightly-alpine AS builder

RUN apk update && apk upgrade --no-cache && apk add --no-cache \
    bash \
    curl \
    git \
    perl \
    make \
    openssl-dev \
    openssl-libs-static \
    binaryen \
    clang \
    lld \
    mold

# Get script to patch and install mimalloc
# RUN git clone "https://github.com/WebVOWL/rust-alpine-mimalloc"

# WORKDIR /rust-alpine-mimalloc

# TESTING
# COPY buildcopy.sh mimalloc.diff ./

# Build latest stable version of mimalloc (2025-06-09)
# RUN /rust-alpine-mimalloc/buildcopy.sh 2.2.4


# WORKAROUND
# WORKDIR /
# Install a prebuilt binary of cargo-leptos matching version in README.md
# RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/download/v0.3.2/cargo-leptos-installer.sh | sh
# COPY cargo-leptos cargo-leptos
# RUN cargo install --debug --path /cargo-leptos/


WORKDIR /work
COPY . .

# Override bin-target-triple defined in Cargo.toml
ENV LEPTOS_BIN_TARGET_TRIPLE="x86_64-unknown-linux-musl"

# TESTING
# Set mimalloc as default memory allocator
# Static build + allocator override may cause issues: 
# https://github.com/microsoft/mimalloc/issues/77#issuecomment-508579118

# Other resources:
# https://github.com/rust-lang/rust/issues/85076

# ENV LD_PRELOAD=/usr/lib/libmimalloc-debug.so

# Build VOWL-R
# RUN cargo leptos build --release --precompress -vv
RUN cargo leptos build -vv


FROM scratch AS runner

WORKDIR /app

USER 10001

# Import VOWL-R from the build stage
COPY --chown=10001 --from=builder /work/target/x86_64-unknown-linux-musl/debug/vowlr /app/
COPY --chown=10001 --from=builder /work/target/site /app/site

# Import the CAcertificates from the build stage to enable HTTPS
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Set CAcertificates directory
ENV SSL_CERT_DIR=/etc/ssl/certs/

# Testing
ENV MIMALLOC_VERBOSE=1
ENV MIMALLOC_SHOW_STATS=1
ENV RUST_BACKTRACE=1

# The delay in N milli-seconds (by default 10) after which mimalloc will purge OS pages that are not in use.
# Setting N to a higher value like 100 can improve performance (sometimes by a lot) at the cost of potentially
# using more memory at times
ENV MIMALLOC_PURGE_DELAY=50

# Set log level for server binary
ENV RUST_LOG="info"

# IP address the server is listening on
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"

# Set directory to serve files from by the server
ENV LEPTOS_SITE_ROOT=./site

# Depends on the port you choose
EXPOSE 8080

# Must match your final server executable name
CMD ["/app/vowlr"]