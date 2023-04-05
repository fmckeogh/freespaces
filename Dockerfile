FROM node as frontend
WORKDIR /workdir/frontend

COPY frontend /workdir/frontend
RUN npm install
RUN npm run build
RUN npm run export

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
RUN cargo build --release --target x86_64-unknown-linux-musl

# test and build app
COPY . .
COPY --from=frontend /workdir/frontend/out /workdir/frontend/out
RUN touch src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
ENV ADDRESS=
ENV DATABASE_URL=
COPY --from=builder /workdir/target/x86_64-unknown-linux-musl/release/freespace .
ENTRYPOINT ["./freespace"]
