✨**doxxer**✨ automates [SemVer](https://semver.org/) versioning using your Git
tags and commit history.

Built in *Rust*, this configurable CLI tool eliminates need for manual version
management and supports you in your project release process.

Inspired by `git describe --tags`, **doxxer** offers tag filtering, custom version
formatting via templates, and diverse version bumping strategies.

![image info](./docs/demo.gif)

# Introduction

## Getting Started

Install `doxxer` via `cargo` or by downloading a binary from the [GitHub Releases](https://github.com/karlis-vagalis/doxxer/releases) page.

**Cargo:**
```bash
cargo install doxxer
```

## Configuration

**doxxer** supports several configuration layers, with later layers overriding
earlier ones:

1. Default values
2. Config files in Current Working Directory (`.doxxer.toml`, then `doxxer.toml`)
3. Specified config file (`--config <file>`) or config file in repo (`--directory <dir>`)
4. Environment variables (e.g., `DOXXER__OUTPUT__FORMAT=json`)
5. Command-line arguments (e.g., `--output json`)

For full details on configuration loading and precedence, see
[Advanced Configuration](./docs/ADVANCED_CONFIGURATION.md).

### Configuration Files

Configuration can be done via `doxxer.toml` or `.doxxer.toml` files using TOML
syntax. These can define global settings or command-specific ones.

*Example*:
```toml
# Global setting
filter.tag = "^v" # Tags that start with "v"

[output]
format = "json"

[next.patch]
increment = 2
```
For detailed structure and file loading logic, refer to the
[Advanced Configuration](./docs/ADVANCED_CONFIGURATION.md).

### Environment Variables

Set environment variables prefixed with `DOXXER__`, using `__` for nesting (e.g.,
`DOXXER__OUTPUT__FORMAT=json`, `DOXXER__NEXT__MAJOR__INCREMENT=2`). See the
[Advanced Configuration](./docs/ADVANCED_CONFIGURATION.md) for a comprehensive
list of variables mapped to config options.

## Usage

**doxxer** is controlled via two main subcommands: `current` and `next`.
Global options affecting all commands can be configured. For a comprehensive list of these options, refer to the [Advanced Configuration](./docs/ADVANCED_CONFIGURATION.md) document or use `doxxer --help`.

The general syntax is:
```bash
doxxer [GLOBAL OPTIONS] <COMMAND> [COMMAND OPTIONS]
```

### Current Version
The `current` command retrieves the latest SemVer version from your Git tags.
```bash
doxxer current
```
You can extract a specific field of the version (e.g., `major`, `minor`) using the `-f, --field` option. For detailed command options, see [Advanced Configuration](./docs/ADVANCED_CONFIGURATION.md) or run `doxxer current --help`.

### Next Version
The `next` command calculates the next SemVer version based on a chosen strategy.
```bash
doxxer next [STRATEGY]
```
If no strategy is specified, it defaults to `dev`. You can use `-f, --field` to extract specific parts of the version. Each strategy (e.g., `major`, `patch`, `prerelease`) has specific options. For detailed information on strategies and their options, consult the [Advanced Configuration](./docs/ADVANCED_CONFIGURATION.md) or run `doxxer next --help`.

## Template Variables
`doxxer` allows for flexible output formatting using templates for the overall version string, prerelease identifiers, and build metadata.

Key templates include:
*   `output.template`: Formats the final version string. Must include `{version}`.
*   `prerelease.template`: Formats the prerelease segment (e.g., `rc.1`).
*   `build_metadata.template`: Formats the build metadata segment (e.g., `build.a1b2c3d`).

Common variables available (depending on the template context) include:
*   `{version}`: The full SemVer string.
*   `{identifier}`: The prerelease identifier (e.g., `alpha`, `rc`).
*   `{inc}`: The prerelease auto-incrementing number.
*   `{distance}`: Commit count since the last tag.
*   `{hash}`: Short commit hash.
*   `{pre}`: Existing prerelease string (useful in `dev` strategy).

For a comprehensive list of variables, their detailed descriptions, and examples, please see the [Template Variables Details](./docs/ADVANCED_CONFIGURATION.md#template-variables-details) in advanced guide.

## Usage Examples

This section provides practical examples of how to use `doxxer` with different configurations.
(Note: `<hash>` in outputs refers to a short commit hash like `a1b2c3d`.)

### 1. Using a `doxxer.toml` Configuration File

Create a `doxxer.toml` file in your project's root directory:

```toml
# Global settings
directory = "."

[filter]
tag = "^v"

[output]
format = "json" # Always output in JSON format
template = "v{version}" # Prefix all versions with "v"

[next.patch]
# Specific settings for 'doxxer next patch'
increment = 5
build_metadata.template = "stable.{hash}"

[next.dev]
# Specific settings for 'doxxer next dev' or 'doxxer next', used for dynamic versions
prerelease.identifier = "post"
prerelease.template = "{pre}.{identifier}.{distance}"
build_metadata.template = "" # No build metadata for this dev version format
```

**With this `doxxer.toml` in place (assuming latest tag matching filter is `v1.3.8-rc.2`):**

*   `doxxer current`:
    ```json
    {
        "full": "v1.3.8-rc.2",
        "major": 1,
        "minor": 3,
        "patch": 8,
        "pre": "rc.2"
    }
    ```

*   `doxxer next patch`:
    ```json
    {
        "build": "stable.99de49a",
        "full": "v1.3.13+stable.99de49a",
        "major": 1,
        "minor": 3,
        "patch": 13
    }
    ```

*   `doxxer next minor`:
    ```json
    {
        "full": "v1.4.0",
        "major": 1,
        "minor": 4,
        "patch": 0
    }
    ```

*   `doxxer next dev` or `doxxer next`:
    ```json
    {
        "full": "v1.3.8-rc.2.post.1",
        "major": 1,
        "minor": 3,
        "patch": 8,
        "pre": "rc.2.post.1"
    }
    ```

### 2. Overriding Settings with CLI Arguments

CLI arguments take precedence over `doxxer.toml` settings.

*   **Override output format to plain text (using `doxxer.toml` from Example 1):**
    ```bash
    doxxer -f plain current
    ```
    Output: `v1.3.8-rc.2`

*   **Override increment for `next patch` (using `doxxer.toml` from Example 1):**
    ```bash
    doxxer next patch --increment 1
    ```
    Output:
    ```json
    {
        "build": "stable.99de49a",
        "full": "v1.3.9+stable.99de49a",
        "major": 1,
        "minor": 3,
        "patch": 9
    }
    ```

*   **Override a output template and format for `next major`:**
    ```bash
    doxxer -o "prod-{version}" -f plain next major
    ```
    Output: `prod-2.0.0`

### 3. Using Environment Variables

Configure `doxxer` without a config file, using environment variables.

*   **Set global output format and `next major` increment:**
    ```bash
    export DOXXER__OUTPUT__FORMAT=json
    export DOXXER__NEXT__MAJOR__INCREMENT=2
    ```

*   **Set a specific prerelease identifier and template for the `dev` strategy:**
    ```bash
    export DOXXER__NEXT__DEV__PRERELEASE_IDENTIFIER="snapshot"
    export DOXXER__NEXT__DEV__PRERELEASE_TEMPLATE="{identifier}.{distance}"
    ```

### 4. Common Use Cases & Specific Scenarios

Assuming latest tag is `v1.3.8-rc.2`:

*   **Always output JSON:**
    *   In `doxxer.toml`:
        ```toml
        [output]
        format = "json"
        ```
    *   Or with env var: `export DOXXER__OUTPUT__FORMAT=json`
    *   Or CLI: `doxxer <command> --format json`

*   **Get only the major version number:**
    ```bash
    doxxer current --field major
    ```
    Output: `1`

*   **Generate `next patch` version relying on defaults:**
    If no `doxxer.toml` exists or it has no `[next.patch]` section:
    ```bash
    doxxer next patch
    ```
    Output: `1.3.9`

*   **Generate dynamic `dev` versions like `1.3.8-rc.2.dev.1+99de49a`:**
    ```bash
    doxxer next
    ```
    *   Output: `1.3.8-rc.2.dev.1+99de49a`

## Docker Support

There is a docker image based on latest `alpine` image and published on GitHub's containter registry: `ghcr.io/karlis-vagalis/doxxer`

### Settings

The default workspace directory inside the container is `/repo`.

### Docker Examples

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

1.  **Q: Why does the default `next dev` strategy sometimes append a new prerelease identifier if one already exists (e.g., `1.0.0-alpha.1` becomes `1.0.0-alpha.1.dev.5`)?**

    A: This behavior is intentional for the `dev` strategy. Its default prerelease template is `"{pre}.{identifier}.{distance}"`. The `{pre}` variable captures the *entire existing* prerelease string. This design allows you to see the lineage from a previous prerelease tag (like `alpha.1`) while also appending the `dev` specific information (identifier and commit distance). It's aimed at providing maximum context during development. If you prefer a different format for `dev` versions, you can customize `next.dev.prerelease.template` and `next.dev.identifier` in your configuration.

2.  **Q: Why is the project called `doxxer`?**

    A: Because it's like somebody who [doxes](https://en.wikipedia.org/wiki/Doxing), by exposing the version information about Git repo to you! Is the name really fitting? Maybe not. But it's just a name.

3.  **Q: Why Rust?**

    A: To learn the language, and Rust offers excellent support for creating binary CLI tools, particularly with libraries like `clap` for argument parsing.

4.  **Q: How do I set an option globally for all commands?**

    A: Global options (like `directory`, `output.format`, `filter.tag`) can be set at the top level of your `doxxer.toml` file or within general tables like `[output]` and `[filter]`. These apply unless overridden by more specific configurations (command-specific, environment variables, or CLI arguments). For environment variables, use the base prefixes (e.g., `DOXXER__OUTPUT__FORMAT`, `DOXXER__FILTER__TAG`).

5.  **Q: What's the easiest way to always output JSON (or another format)?**
    A: The most persistent way is to set it in your `doxxer.toml` file:
    ```toml
    [output]
    format = "json"
    ```
    Alternatively, you can export an environment variable: `export DOXXER__OUTPUT__FORMAT=json`. If you only need it for a single command execution, use the `-f json` (or `--format json`) CLI flag.

6.  **Q: Can I have different prerelease identifiers for `patch` vs. `minor` bumps when using strategies like `prepatch` or `preminor`?**
    A: Yes! You can define this in your `doxxer.toml` by targeting the specific strategy:
    ```toml
    [next.pre-patch]
    prerelease.identifier = "rc" # e.g., for release candidates on patches, version like 1.2.3-rc.1

    [next.pre-minor]
    prerelease.identifier = "beta" # e.g., for beta releases on minor bumps, version like 1.3.0-beta.1
    ```

# Roadmap

- [ ] Add installation shell script, similar to `uv` or `just` to install prebuild binaries from GitHub
- [ ] Add `{timestamp}` variable support to the template