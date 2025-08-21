# FROM rustlang/rust:nightly-alpine AS builder

# RUN apk update && apk add --no-cache \
#     bash \
#     curl \
#     binaryen \
#     clang \
#     lld

# # Install a prebuilt binary of cargo-leptos
# RUN curl --proto '=https' --tlsv1.3 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# WORKDIR /work
# COPY . .

# # Override bin-target-triple defined in Cargo.toml
# ENV LEPTOS_BIN_TARGET_TRIPLE="x86_64-unknown-linux-musl"

# # Build WebVOWL
# RUN cargo leptos build --release --precompress -vv


# FROM scratch AS runner

# WORKDIR /app

# USER 10001

# COPY --chown=10001 --from=builder /work/target/x86_64-unknown-linux-musl/release/webvowl-reimagined /app/
# COPY --chown=10001 --from=builder /work/target/site /app/site

# ENV RUST_LOG="info"
# ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
# ENV LEPTOS_SITE_ROOT=./site

# # Depends on the port you choose
# EXPOSE 8080

# # Must match your final server executable name
# CMD ["/app/webvowl-reimagined"]
##########################################
FROM rustlang/rust:nightly-slim AS builder

RUN apt-get update -y && apt-get install -y \
    wget \
    binaryen \
    clang \
    lld

# Install cargo-binstall, which makes it easier to install other cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

WORKDIR /work
COPY . .

# Build WebVOWL
RUN cargo leptos build --release --precompress -vv


FROM scratch AS runner

WORKDIR /app

COPY --from=builder /work/target/x86_64-unknown-linux-gnu/release/webvowl-reimagined /app/
COPY --from=builder /work/target/site /app/site

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site

# Depends on the port you choose
EXPOSE 8080

# Must match your final server executable name
CMD ["/app/webvowl-reimagined"]