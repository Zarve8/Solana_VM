FROM rust:1.75.0

WORKDIR /app


COPY contracts/ contracts/
COPY data-manager/ data-manager/
COPY executor/ executor/
COPY piper/ piper/
COPY program-manager/ program-manager/
COPY sdk/ sdk/
COPY spawner/ spawner/
COPY src/ src/
COPY super-lib/ super-lib/
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY shel/ shel/

RUN cargo build --package super-vm
RUN cargo build --package system-program && cp target/debug/libsystem_program.so target/debug/lib11111111111111111111111111111111.so
RUN cargo build --package spl-token && cp target/debug/libspl_token.so target/debug/libTokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA.so
RUN cargo build --package test-contract && cp target/debug/libtest_contract.so target/debug/libFaGHKV74yrwsAgbp9SxadKhBbfAQteNbJEetsEWUppCa.so
RUN cargo build --package spawner
RUN cargo build --package executor

RUN cargo test --package data-manager

ENTRYPOINT [ "cargo", "run" ]

