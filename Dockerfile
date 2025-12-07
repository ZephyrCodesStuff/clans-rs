FROM rust:slim AS builder

RUN apt-get update -y && \
  apt-get install -y pkg-config make g++ libssl-dev && \
  rustup target add x86_64-unknown-linux-gnu

WORKDIR /app
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-gnu

# ------------------------------

FROM gcr.io/distroless/cc

WORKDIR /app

COPY --from=builder /app/keys /app/keys
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/clans-rs /app/clans-rs

ENTRYPOINT [ "/app/clans-rs" ]
