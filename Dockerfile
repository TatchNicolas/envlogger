FROM ekidd/rust-musl-builder:latest as builder
ADD --chown=rust:rust . ./
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release

FROM gcr.io/distroless/base-debian10
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/envlogger \
    /usr/local/bin/
CMD ["envlogger"]
