# Build stage
FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new simload_rust
WORKDIR /usr/src/simload_rust
COPY Cargo.toml Cargo.lock messages.txt ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Bundle Stage
FROM scratch
COPY --from=builder /usr/local/cargo/bin/simload .
USER 1000
EXPOSE 8000
CMD ["./simload"]
