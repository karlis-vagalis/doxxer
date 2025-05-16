container := "ghcr.io/karlis-vagalis/doxxer"

default-tag := "latest"
current-tag := `cargo run -- -o "" current`

build-linux-gnu:
    docker run --rm --volume .:/root/src --workdir /root/src joseluisq/rust-linux-darwin-builder:1.85 sh -c "cargo build --release --target x86_64-unknown-linux-gnu"
build-linux-musl:
    docker run --rm --volume .:/root/src --workdir /root/src joseluisq/rust-linux-darwin-builder:1.85 sh -c "cargo build --release --target x86_64-unknown-linux-musl"

build-release: && build-linux-gnu build-linux-musl

build-docker:
    docker buildx build -t {{container}}:{{default-tag}} -t {{container}}:{{current-tag}} .

run-docker *args:
    docker run --rm -v .:/repo -it {{container}}:{{default-tag}} {{args}}

publish-docker: build-docker
    docker push {{container}}:{{default-tag}}
    docker push {{container}}:{{current-tag}}