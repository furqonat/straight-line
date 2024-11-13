FROM rust:1.82.0-alpine3.20 AS builder

WORKDIR /auth

### add dependencies
RUN apk add --no-cache musl-dev openssl-dev

COPY . .

RUN cargo build --release

FROM alpine:3.20

WORKDIR /auth

COPY --from=builder /auth/dist/target/release/auth /auth
COPY --from=builder /auth/.env ./

RUN chmod +x /auth

#install curl
RUN apk add --no-cache curl

EXPOSE 8080

CMD ["./auth"]