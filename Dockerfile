FROM rust:1.40-alpine

WORKDIR /usr/src/league_rankings
COPY . .

RUN cargo install --path .
CMD ["league_rankings"]
