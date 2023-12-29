# Start with a Rust base image
FROM rust:latest as builder

# Create a new empty Rust project
RUN USER=root cargo new --bin api-rust
WORKDIR /api-rust

# Copy the Cargo.toml and Cargo.lock and build the dependencies
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Define a build argument for the database URL
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}

# Copy the source code and build the application
COPY ./src ./src
COPY ./prisma ./prisma
COPY ./prisma-cli ./prisma-cli
COPY ./.cargo ./.cargo
RUN cargo prisma generate
RUN cargo build --release

# Use a minimal Debian image for the runtime
FROM ubuntu:latest

RUN apt-get update -y
RUN apt-get install -y openssl
RUN apt-get install -y libssl-dev
RUN apt-get install -y ca-certificates


# Copy the build artifact from the builder stage
COPY --from=builder api-rust/target/release/api-rust /usr/local/bin

# Expose the port your application runs on
EXPOSE 3000

# Set the command to run your application
CMD ["api-rust"]
