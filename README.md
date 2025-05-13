`Doxxer` is a CLI tool written in Rust to automatically generate dynamic
[SemVer](https://semver.org/) versions based on the latest Git tags and commits.

This approach was heavily inspired by the output produced by the native
`git decribe --tags`, with adjustments to comply with specification.

# Introduction

## Getting started

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

## Examples

### Getting current version

```
doxxer current
```

### Getting upcoming version

#### With tag prefix

```
doxxer next
```

#### Without tag prefix

```
doxxer -p "" next
```