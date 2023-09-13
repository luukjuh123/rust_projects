FROM rust:latest as builder

WORKDIR /usr/src/myfrontend
COPY . .

RUN cargo build --release


FROM debian:buster-slim

COPY --from=builder /usr/src/myfrontend/target/release/myfrontend /usr/local/bin/myfrontend

COPY ./source/test.txt /source/test.txt

CMD ["myfrontend", "/source/test.txt"]
