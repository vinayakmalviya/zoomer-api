FROM rust:1.69-bullseye as builder

# Creating an empty cargo project
RUN USER=root cargo new --bin zoomer_api
WORKDIR /zoomer_api

# Copying Cargo.toml and lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Caching deps
RUN cargo build --release
RUN rm src/*.rs

# Copying src
COPY ./src ./src

# Building final release binary
RUN rm ./target/release/deps/zoomer_api*
RUN cargo build --release

# Final image
FROM debian:bullseye

# Copying release binary
COPY --from=builder /zoomer_api/target/release/zoomer_api .

# Running the binary
CMD [ "./zoomer_api" ]
