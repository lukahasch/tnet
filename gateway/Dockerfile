FROM rust:1.72 
RUN git clone https://github.com/henry3141/tnet
WORKDIR /tnet/gateway
RUN cargo build --release
CMD ["./target/release/gateway"]
