FROM rust:latest
LABEL author="revanth"

WORKDIR /usr/src/iam-service
COPY . .

RUN cargo build --release

CMD ["./target/release/iam-service"]