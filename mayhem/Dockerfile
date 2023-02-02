FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD ../ /railwind
WORKDIR /railwind/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /railwind/fuzz/target/x86_64-unknown-linux-gnu/release/railwind-fuzz /