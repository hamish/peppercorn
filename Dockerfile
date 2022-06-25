FROM rust:1.59.0 as builder

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

ENV SQLX_OFFLINE true 

RUN cargo build --release

FROM rust:1.59.0-slim as runtime

WORKDIR /app

COPY --from=builder /app/target/release/peppercorn peppercorn
COPY configuration configuration
ENV APP_ENVIRONMENT production

ENTRYPOINT [".peppercorn"]
