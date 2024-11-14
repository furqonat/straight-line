FROM rust:1.82.0-alpine3.20 AS builder

WORKDIR /app

### add dependencies
RUN apk add --no-cache musl-dev openssl-dev

COPY . .

RUN cargo build --release

FROM alpine:3.20

WORKDIR /app

COPY --from=builder /app/dist/target/release/user /app
COPY --from=builder /app/.env ./

RUN chmod +x ./user

#install curl
RUN apk add --no-cache curl

EXPOSE 8080

CMD ["./user"]