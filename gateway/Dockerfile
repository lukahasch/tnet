FROM rust:1.72 
WORKDIR /main
RUN git clone https://github.com/henry3141/tnet
WORKDIR /main/tnet/gateway
RUN cargo build --release
CMD ["./target/release/gateway"]
