FROM rust:latest

RUN rustup target add arm-unknown-linux-gnueabihf
RUN apt-get update &&\
	apt-get install -qq gcc-arm-linux-gnueabihf \
						libc6-armel-cross \
						libc6-dev-armel-cross

RUN mkdir -p ~/.cargo
RUN echo "[target.arm-unknown-linux-gnueabihf]\nlinker = \"arm-linux-gnueabihf-gcc\"" > ~/.cargo/config