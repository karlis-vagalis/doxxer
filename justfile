build-linux-gnu:
    docker run --rm --volume .:/root/src --workdir /root/src joseluisq/rust-linux-darwin-builder:1.85 sh -c "cargo build --release --target x86_64-unknown-linux-gnu"
build-linux-musl:
    docker run --rm --volume .:/root/src --workdir /root/src joseluisq/rust-linux-darwin-builder:1.85 sh -c "cargo build --release --target x86_64-unknown-linux-musl"
build-apple-darwin:
    docker run --rm --volume .:/root/src --workdir /root/src joseluisq/rust-linux-darwin-builder:1.85 sh -c "CC=o64-clang CXX=o64-clang++ cargo build --release --target x86_64-apple-darwin"

build-release: && build-linux-gnu build-linux-musl build-apple-darwin