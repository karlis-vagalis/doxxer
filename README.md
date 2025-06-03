✨**doxxer**✨ is a highly configurable CLI tool written in *Rust* that
simplifies and automates dynamic [SemVer](https://semver.org/) versioning by
leveraging the latest Git tags and commits.

It includes tag filtering, templating mechanisms, different output formats, many
version bumping mechanisms and is the perfect tool to generate dynamic version
on the fly. No more project programming language specific solution to get the
version of the software your are developing!

This approach was heavily inspired by the output produced by the native
`git decribe --tags`, which was my own de-facto solution for dynamic versioning of
software projects.

![image info](./docs/demo.gif)

# Introduction

## Getting started

You can currenly install the tool locally via *cargo*:

```bash
cargo install doxxer
```

or downloading latest binary from [GitHub Release](https://github.com/karlis-vagalis/doxxer/releases) page.

## Configuration

The tool supports

## Usage

The tool has 2 main commands to work with your repository: `current` and `next`.

```bash
doxxer help
```

```bash
Dynamic version manager for Git repositories

Usage: doxxer [OPTIONS] <COMMAND>

Commands:
  current  Get current version
  next     Get next version
  help     Print this message or the help of the given subcommand(s)

Options:
  -d, --directory <PATH>  Path to the Git repository
  -c, --config <PATH>     Path to the config file or directory
  -h, --help              Print help
  -V, --version           Print version

Filter options:
  -t, --tag-filter <REGEX>  Regular expression for selecting relevant tags [default: ]

Output options:
  -f, --format <FORMAT>      Output format [default: plain] [possible values: plain, json]
  -o, --template <TEMPLATE>  Template for resulting version [default: {version}]
```

## Current version


```
doxxer current --help
```

```bash
Get current version

Usage: doxxer current [OPTIONS]

Options:
  -f, --field <FIELD>  Field/part of the version [possible values: major, minor, patch, prerelease, build-metadata]
  -h, --help           Print help
```

## Next version

```
doxxer next --help
```

```bash
Get next version

Usage: doxxer next [OPTIONS] [STRATEGY]

Bumping strategy:
  major       Major version
  minor       Minor version
  patch       Patch version
  prerelease  Pre-release version
  pre-major   Major + pre-release version
  pre-minor   Minor + pre-release version
  pre-patch   Patch + pre-release version
  dev         Development version (non-standard)
  help        Print this message or the help of the given subcommand(s)

Options:
  -f, --field <FIELD>  Field/part of the version [possible values: major, minor, patch, prerelease, build-metadata]
  -h, --help           Print help
```

Default behaviour/strategy of the `next` command is `dev` ideal for generating development versions dynamically.

## Template variables

### `output.template`

| Variable | Description |
|--|--|
| `{version}` | SemVer string. Required |

### `template` (prerelease & build metadata)

| Variable | Description |
|--|--|
| `{pre}` | Current pre-release |
| `{identifier}` | Name of the pre-release identifier |
| `{inc}` | Next version for specified `{identifier}` |
| `{hash}` | First 7-digits of the commit hash |
| `{distance}` | Count of commits since last tag |

# Docker

There is a docker image based on latest `alpine` image and published on GitHub's containter registry: `ghcr.io/karlis-vagalis/doxxer`

## Settings

The default workspace directory inside the container is `/repo`.

The entrypoint is `doxxer`.


## Examples

To execute `doxxer next` command once, you could run:

```bash
docker run --rm -v .:/repo -it ghcr.io/karlis-vagalis/doxxer:latest doxxer next
```

where, we mount current (`.`) directory inside the container and execute `next` subcommand.

## As base image

If you want to include *doxxer* binary inside your custom docker image, you can copy the binary like so:

```dockerfile
FROM ghcr.io/karlis-vagalis/doxxer:latest AS base
...
COPY --from=base /bin/doxxer /bin
...
```

where we copy the binary to the `/bin` folder in our new image.

# FAQ

1. Why is does the default strategy for `next` appends second pre-release?

Because the original idea and goal of this tool is to dynamically generate version
for a project, when iterating/developing. So, the defaults reflect this goal and
thus, require least amount of configuration.

2. Why is the project called `doxxer`?

Because it's like somebody who [doxes](https://en.wikipedia.org/wiki/Doxing), by
exposing the version information about Git repo to you! It the name really fitting? Maybe no. It's just a name.

3. Why rust?

To learn the language and Rust offers excellent support for creating binary CLI tools
using `clap`.

# Roadmap

- [ ] Add installation shell script, similar to `uv` or `just` to install prebuild binaries from GitHub
- [ ] Add `{timestamp}` variable support to the template
