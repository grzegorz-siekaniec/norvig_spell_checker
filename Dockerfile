FROM rust as build

WORKDIR /home/app

COPY ./ ./

RUN cargo build --release --jobs 6 \
    && cargo test \
    && mkdir bin \
    && cp target/release/norvig_spell_checker bin/ \
    && cp .env bin/ \
    && cp Cargo.toml bin/ \
    && cp -r data/ bin/

FROM debian:buster-slim

COPY --from=build /home/app/bin /home/app

# .env is located here to make sure docker reads this value and it reads it from current working directory
WORKDIR /home/app

CMD ["./norvig_spell_checker", "run", "--corpus", "data/big.txt"]
