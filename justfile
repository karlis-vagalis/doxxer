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

build-release:
    just build "x86_64-unknown-linux-gnu"
    just build "x86_64-unknown-linux-musl"

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

release token:
    curl -L \
        -X POST \
        -H "Accept: application/vnd.github+json" \
        -H "Authorization: Bearer {{ token }}" \
        -H "X-GitHub-Api-Version: 2022-11-28" \
        https://api.github.com/projects/{{owner}}/{{project}}/releases \
        -d '{"tag_name":"{{current-tag}}","target_commitish":"master","name":"{{current-tag}}","body":"","draft":false,"prerelease":false,"generate_release_notes":false}'

encapsulate triple:
    #!/usr/bin/env bash
    set -euxo pipefail
    name="{{project}}-{{triple}}"
    mkdir $name
    cp ./target/{{triple}}/release/{{project}} ./$name
    tar -czvf $name.tar.gz $name
    rm -r $name

assets:
    just encapsulate "x86_64-unknown-linux-gnu"
    just encapsulate "x86_64-unknown-linux-musl"