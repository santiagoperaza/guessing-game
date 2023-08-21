# Builder stage
FROM rust:1.71.0-alpine AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# Runtime stage
FROM rust:1.71.0-alpine AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/guessing-game guessing-game

ENTRYPOINT ["./guessing-game"]