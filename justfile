registry := "ghcr.io"
owner := "karlis-vagalis"
project := "doxxer"
image := registry + "/" + owner + "/" + project

default-tag := "latest"
current-tag := `cargo run -- current`

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