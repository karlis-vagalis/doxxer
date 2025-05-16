registry := "ghcr.io"
owner := "karlis-vagalis"
project := "doxxer"
image := registry + "/" + owner + "/" + project

default-tag := "latest"
current-version := `cargo run -- -o "" current`
current-tag := `cargo run -- current`

build triple:
    docker run --rm --volume .:/root/src \
        --workdir /root/src joseluisq/rust-linux-darwin-builder:1.85 \
        sh -c "cargo build --release --target {{triple}}"

encapsulate triple:
    #!/usr/bin/env bash
    set -euxo pipefail
    name="{{project}}-{{triple}}"
    mkdir $name
    cp ./target/{{triple}}/release/{{project}} ./$name
    tar -czvf $name.tar.gz $name
    rm -r $name

assets triple:
    just build {{triple}}
    just encapsulate {{triple}}

build-release:
    just assets "x86_64-unknown-linux-gnu"
    just assets "x86_64-unknown-linux-musl"

build-docker:
    docker buildx build \
        -t {{image}}:{{default-tag}} \
        -t {{image}}:{{current-version}} \
        .

run-docker *args:
    docker run --rm -v .:/repo -it {{image}}:{{default-tag}} {{args}}

publish-docker: build-docker
    docker push {{image}}:{{default-tag}}
    docker push {{image}}:{{current-version}}

clean:
    rm -f *.tar.gz