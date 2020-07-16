FROM rust:1.44.1

RUN apt-get update \
    && apt-get install -y wait-for-it=0.0~git20180723-1 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /work
WORKDIR /work
# for cache
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src/
RUN echo "fn main() {println!(\"broken\")}" > src/main.rs \
    && cargo build --release

COPY ./src/ ./src/
RUN cargo build --release

CMD ["/work/target/release/rust-mysql-redis-client-usage"]
