FROM rust:1.75-bookworm as build

RUN apt-get update
RUN apt-get install protobuf-compiler -y

# create a new empty shell project
WORKDIR /app

# copy your source tree
COPY src ./src

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# build for release
RUN cargo build --release

# our final base
FROM debian:bookworm

# copy the build artifact from the build stage
COPY --from=build /app/target/release/rust-numa-map .

# set the startup command to run your binary
CMD ["./rust-numa-map"]