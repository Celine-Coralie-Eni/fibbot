#Stage 1: Build the Rust application*
FROM rust:1.70 AS build

#Set the working directory*
WORKDIR /usr/src/myapp

#Copy the Cargo.toml and Cargo.lock files*
COPY Cargo.toml Cargo.lock ./

#Create a new empty shell project to cache dependencies*
RUN mkdir src && echo "fn main() {}" > src/main.rs

#Build the dependencies*
RUN cargo build --release
RUN rm -r src

#Copy the actual source code*
COPY . .

#Build the application*
RUN cargo build --release

#Stage 2: Create a minimal runtime image*
FROM alpine:latest

#Install necessary dependencies*
RUN apk add --no-cache libgcc

#Set the working directory*
WORKDIR /usr/local/bin

#Copy the compiled binary from the builder stage*
COPY --from=builder /usr/src/myapp/target/release/myapp .

#Command to run the application*
CMD ["./myapp"]




# FROM rust:1.67 AS build
# COPY . .
# RUN rustup target add x86_64-unknown-linux-musl
# RUN cargo install --path . --target x86_64-unknown-linux-musl

# FROM alpine:3.16.0 AS runtime
# COPY --from=build /usr/local/cargo/bin/myapp /usr/local/bin/myapp

# FROM runtime as action
# COPY entrypoint.sh /entrypoint.sh

# ENTRYPOINT [ /entrypoint.sh ]
