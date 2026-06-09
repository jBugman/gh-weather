FROM rust:1.96-slim AS builder
WORKDIR /build
COPY . .
RUN cargo build --release --features prod && \
    strip target/release/gh-weather

FROM rust:1.96-slim
COPY --from=builder /build/target/release/gh-weather /usr/local/bin/
CMD ["gh-weather"]
