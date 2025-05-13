`Doxxer` is a CLI tool written in Rust to automatically generate dynamic
[SemVer](https://semver.org/) versions based on the latest Git tags and commits.

This approach was heavily inspired by the output produced by the native
`git decribe --tags`, with adjustments to comply with specification.

# Introduction

## Getting started

To install from cargo:

```bash
cargo install doxxer
```

## Usage

```bash
doxxer
```

```
Dynamic version manager for Git

Usage: doxxer [OPTIONS] <COMMAND>

Commands:
  current  Returns current version string from latest tag
  next     Returns next version string
  help     Print this message or the help of the given subcommand(s)

Options:
  -d, --directory <DIRECTORY>    Path to the Git repository [default: .]
  -t, --tag-prefix <TAG_PREFIX>  Prefix of the tag names used for releases [default: v]
  -h, --help                     Print help
  -V, --version                  Print version

Output options:
  -p, --prefix <PREFIX>  Add tag prefix to the output version [default: v]
```

## Next version

```
doxxer next --help
```

```
Returns next version string

Usage: doxxer next [OPTIONS] [COMMAND]

Commands:
  major  Get major version
  minor  Get minor version
  patch  Get patch version
  pre    Get pre-release version
  build  Get build metadata
  help   Print this message or the help of the given subcommand(s)

Options:
  -s, --strategy <STRATEGY>
          Bumping strategy [default: pre-build] [possible values: major, minor, patch, pre-build]
  -p, --pre-template <PRE_TEMPLATE>
          Template for next version's pre-release [default: {pre}.dev.{distance}]
  -b, --build-template <BUILD_TEMPLATE>
          Template for next version's build metadata [default: {hash}]
  -h, --help
          Print help
```

## Template variables

| Variable | Description |
|--|--|
| `{pre}` | Current pre-release |
| `{hash}` | First 7-digits of the commit hash |
| `{distance}` | Count of commits since last tag |

## Examples

Lets assume, we do not have any Git tag with sematic version yet, thus, `0.0.0`
is the fallback. We are `19` commits past the origin, with commit hash `1b9f41e`.

### Getting current version

```
doxxer current
```
*Output*: `v0.0.0`

#### Major version only

```
doxxer current major
```
*Output*: `0`

### Getting upcoming version

#### Patch version

```
doxxer next -s patch
```
*Output*: `v0.0.1`

#### With tag prefix

```
doxxer next
```
*Output*: `v0.0.0-dev.19+1b9f41e`

#### With custom tag prefex

```
doxxer next -p "release-"
```
*Output*: `release-0.0.0-dev.19+1b9f41e`

#### Without tag prefix

```
doxxer -p "" next
```
*Output*: `0.0.0-dev.19+1b9f41e`