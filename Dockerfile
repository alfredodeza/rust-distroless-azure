FROM rust:1.85-bookworm as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

# Now copy it into our base image.
FROM gcr.io/distroless/cc-debian12:nonroot

COPY --from=builder /usr/src/app/target/release/rust-tokenizers-api /usr/local/bin/rust-tokenizers-api
EXPOSE 8000
USER nonroot:nonroot
ENTRYPOINT ["/usr/local/bin/rust-tokenizers-api"]