FROM rustlang/rust:nightly AS builder

WORKDIR /workdir
ENV CARGO_HOME=/workdir/.cargo
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
COPY ./sink ./sink
RUN cargo +nightly build --release

FROM debian:stretch-slim
EXPOSE 3030
COPY --from=0 /workdir/target/release/backend /usr/local/bin
ENTRYPOINT ["/usr/local/bin/backend"]