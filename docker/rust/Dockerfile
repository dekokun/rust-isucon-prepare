FROM rust:1.44.1

# for cache
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p /src
WORKDIR /src
RUN echo "fn main() {println!(\"broken\")}" > /src/main.rs \
    && cargo build --release

COPY . .
RUN cargo build --release
RUN cargo install --path .

CMD ["/usr/local/cargo/bin/rust-mysql-redis-client-usage"]
