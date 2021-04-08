# Based on https://alexbrand.dev/post/how-to-package-rust-applications-into-minimal-docker-containers/
# and https://www.artificialworlds.net/blog/2020/04/22/creating-a-tiny-docker-image-of-a-rust-project/
# 1: Build the exe
FROM rust:1.51 as builder
WORKDIR /usr/src

# 1a: Prepare for static linking
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl

# 1b: Download and compile Rust dependencies (and store as a separate Docker layer)
RUN USER=root cargo new tcp_server
WORKDIR /usr/src/tcp_server
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# 1c: Build the exe using the actual source code
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# 2: Copy the exe and extra files ("static") to an empty Docker image
FROM scratch
COPY --from=builder /usr/local/cargo/bin/tcp_server .
USER 1000
ENTRYPOINT ["./tcp_server"]