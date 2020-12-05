# --- Use custom rust-musl-builder image
FROM ekidd/rust-musl-builder as BUILDER

# Add the Rust-Lang source-code
ADD --chown=rust:rust . ./

# Build the application
RUN cargo build --release

# --- Build the deployment container
FROM gliderlabs/alpine:latest

# Copy compiled musl binary
COPY --from=BUILDER \
    ./home/rust/src/target/x86_64-unknown-linux-musl/release/rust-template \
    ./

# --- Run Rust application
CMD /rust-template