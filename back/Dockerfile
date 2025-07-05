FROM rust:1.88

WORKDIR /back

COPY . .

RUN cargo build --release

CMD ["./target/release/back"]
