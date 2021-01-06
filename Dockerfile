FROM rust:nightly

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

RUN USER=root cargo new --bin rocket-api
WORKDIR /rocket-api
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build

RUN rm src/*.rs
COPY ./src ./src
RUN rm ./target/debug/deps/rocket_api*
RUN cargo build

CMD ["./target/debug/rocket-api"]
#CMD ["cargo", "run"]
#EXPOSE 8000
