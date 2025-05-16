FROM alpine
COPY ./target/x86_64-unknown-linux-musl/release/doxxer /bin
WORKDIR /repo
CMD ["doxxer", "help"]