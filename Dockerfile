#FROM rust:latest as builder
FROM ekidd/rust-musl-builder as builder


WORKDIR /app

COPY . .

RUN cargo build --release

#FROM debian:buster-slim
FROM scratch

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/server /app
COPY --from=builder /app/adorable/img /app/adorable/img


ENV PORT 8080

ENV RUST_LOG "warn,err"

EXPOSE $PORT

CMD ["./server"]
