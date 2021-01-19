FROM rust:nightly as builder

RUN USER=root cargo new --bin rust-microservices
WORKDIR /rust-microservices
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build

RUN rm src/*.rs
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml ./diesel.toml
RUN rm ./target/debug/deps/rust_microservices*
RUN cargo build

FROM buildpack-deps:stretch

COPY --from=builder /rust-microservices/target/debug/rust-microservices /app/

ENTRYPOINT [ "/app/rust-microservices" ]
