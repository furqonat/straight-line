FROM rust:1.66.1-alpine3.16 as builder

WORKDIR /auth

COPY . .

RUN cargo build --release

FROM alpine:3.16

WORKDIR /auth

COPY --from=builder /auth/dist/target/release/auth /auth

CMD ["./auth"]