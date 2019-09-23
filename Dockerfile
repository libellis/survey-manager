FROM rust:1.37.0-stretch as builder

# muslc is required in order to build the rust image.
RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*

COPY . .
RUN rustup target add x86_64-unknown-linux-musl
# Sets the environment variable for the cargo build command that follows.
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.10

RUN apk --no-cache add ca-certificates 

# Copy build files over
COPY --from=builder /target/x86_64-unknown-linux-musl/release/survey-manager-api .

CMD ["./survey-manager-api"]
