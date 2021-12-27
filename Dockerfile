FROM rust:1.57 as build

# create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# copy over your manifests
# ADD ./Cargo.lock ./Cargo.lock
ADD ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
ADD ./src ./src

# build for release
RUN rm ./target/release/deps/app*
RUN cargo build --release

# our final base
# FROM gcr.io/distroless/cc-debian11
FROM rust:1.57-slim-buster

# copy the build artifact from the build stage
COPY --from=build /app/target/release/app .

# set the startup command to run your binary
EXPOSE 8080

CMD ["./app"]