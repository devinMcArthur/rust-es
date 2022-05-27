# Source: https://github.com/tmsdev82/rust-warp-docker-tutorial
FROM rust:1.61.0-alpine as builder

# create a new empty shell project
RUN USER=root cargo new --bin server
WORKDIR /server

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/server*
RUN cargo build --release

# our final base
FROM rust:1.61.0-slim-buster

# copy the build artifact from the build stage
COPY --from=builder /server/target/release/server .

# set the startup command to run your binary
CMD ["./server"]