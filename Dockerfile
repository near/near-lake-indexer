FROM rust:1.81 AS builder
WORKDIR /tmp/

# Copy from nearcore:
# https://github.com/near/nearcore/blob/master/Dockerfile
RUN apt-get update -qq && \
    apt-get install -y \
        git \
        cmake \
        g++ \
        pkg-config \
        libssl-dev \
        curl \
        llvm \
        clang

COPY . .

# build for release
RUN cargo build --release

FROM debian:bookworm-slim as runtime
WORKDIR /near-lake-app

RUN apt update && apt install -yy openssl ca-certificates jq

COPY --from=builder /tmp/target/release/near-lake .
COPY ./entrypoint.sh entrypoint.sh
ENTRYPOINT [ "./entrypoint.sh" ]
