FROM mcr.microsoft.com/devcontainers/rust:2-1-trixie

# setup rust toolchain
RUN rustup target add armv7-unknown-linux-gnueabi

# add linker
RUN apt update && apt install -y build-essential gcc-arm-linux-gnueabi