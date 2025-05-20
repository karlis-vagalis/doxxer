registry := "ghcr.io"
owner := "karlis-vagalis"
project := "doxxer"
image := registry + "/" + owner + "/" + project

default-tag := "latest"
current-version := `cargo run -- -o "" current`
current-tag := `cargo run -- current`

build-cross triple:
    cross build --release --target {{triple}}

build-windows:
    docker run --rm -it -v $(pwd):/io -w /io messense/cargo-xwin cargo xwin build --release --target x86_64-pc-windows-msvc

encapsulate triple extension:
    #!/usr/bin/env bash
    set -euxo pipefail
    name="{{project}}-{{triple}}"
    mkdir $name
    cp ./target/{{triple}}/release/{{project}}{{extension}} ./$name
    tar -czvf $name.tar.gz $name
    rm -r $name

build-release:
    just build-cross "x86_64-unknown-linux-gnu"
    just build-cross "x86_64-unknown-linux-musl"
    just build-windows

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