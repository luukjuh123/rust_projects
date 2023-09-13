FROM rustlang/rust:nightly AS builder

WORKDIR /workdir
ENV CARGO_HOME=/workdir/.cargo
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
COPY ./source ./source
RUN cargo +nightly build --release

FROM debian:stretch-slim
EXPOSE 8000
COPY --from=0 /workdir/target/release/frontend /usr/local/bin
ENTRYPOINT ["/usr/local/bin/frontend"]