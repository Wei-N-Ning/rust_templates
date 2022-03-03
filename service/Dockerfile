# Dockerfile for creating a statically-linked Rust application using docker's
# multi-stage build feature. This also leverages the docker build cache to avoid
# re-downloading dependencies if they have not changed.
FROM rust:1.59.0 AS build
WORKDIR /usr/src

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
# the name of the dummy project should match the package name
# `rust_docker_demo`
RUN USER=root cargo new project
WORKDIR /usr/src/project

# Copy the source and build the application.
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Copy the statically-linked binary into a scratch container.
# the name of the binary is 'service'. see Cargo.toml
# also note the service listens to 8080 - make sure it is
# mapped to a host port, e.g.
# docker run -p 8080:8080 rust_templates_service
# 
# there is a drawback: I can not ctrl+c to stop the service; have to
# call `docker kill` - it needs a better shutdown system.
FROM scratch
COPY --from=build /usr/local/cargo/bin/service .
USER 1000
ENTRYPOINT ["./service"]
