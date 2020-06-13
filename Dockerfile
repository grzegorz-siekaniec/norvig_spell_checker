FROM rust as build

COPY ./ ./

RUN cargo build --release \
    && cargo test \
    && mkdir -p /app \
    && cp target/release/norvig_spell_checker /app \
    && cp .env /app \
    && cp -r /data /app/

FROM debian:buster-slim

COPY --from=build /app /home

# .env is located here to make sure docker reads this value and it reads it from current working directory
WORKDIR /home

CMD ["./norvig_spell_checker", "run", "--corpus", "/data/big.txt"]
