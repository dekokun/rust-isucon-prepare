FROM rust:1.44.1

# for cache
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p /src \
    && echo "fn main() {println!(\"broken\")}" > /src/main.rs \
    && cargo build --release
WORKDIR /src

COPY . .
RUN cargo build --release
RUN cargo install --path .

CMD ["ls", "/usr/local/cargo/bin"]
