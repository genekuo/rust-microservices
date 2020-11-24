FROM rust:nightly

RUN USER=root cargo new --bin rust-microservices
WORKDIR /rust-microservices
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build

RUN rm src/*.rs
COPY ./src ./src
RUN rm ./target/debug/deps/rust_microservice*
RUN cargo build

CMD ["./target/debug/rust-microservice"]

EXPOSE 8080
