FROM rust AS builder
WORKDIR /workdir

# prepare toolchain
RUN rustup target add x86_64-unknown-linux-musl

# add musl tools
RUN apt-get update && apt-get install musl-tools clang llvm -y

# build dependencies
RUN cargo init --bin .
COPY Cargo.lock .
COPY Cargo.toml .
RUN cargo build --tests
RUN cargo build --release --target x86_64-unknown-linux-musl

# test and build app
COPY src src
COPY frontend frontend
RUN touch src/main.rs
RUN cargo test
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine
ENV ADDRESS=
ENV DATABASE_URL=
COPY --from=builder /workdir/target/x86_64-unknown-linux-musl/release/freespace .
ENTRYPOINT ["./freespace"]
