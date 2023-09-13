FROM rust:latest as builder
WORKDIR /usr/src/mybackend
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/mybackend /usr/local/bin/mybackend
CMD ["mybackend"]
