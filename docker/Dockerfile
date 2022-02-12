FROM rust:latest as build-image

WORKDIR /rust/docker
COPY src/ /rust/docker/src/
COPY Cargo.toml Cargo.lock /rust/docker/

RUN apt-get update && apt-get install -y curl libssl-dev pkg-config build-essential
RUN rustup update && \
    rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM public.ecr.aws/lambda/provided:al2

COPY  --from=build-image /rust/docker/target/x86_64-unknown-linux-musl/release/docker-lambda-rust-builder ${LAMBDA_RUNTIME_DIR}/bootstrap

CMD [ "lambda-handler" ]