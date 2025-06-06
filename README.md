✨**doxxer**✨ automates [SemVer](https://semver.org/) versioning using your Git tags and commit history. Built in *Rust*, this configurable CLI tool eliminates manual versioning headaches and ensures consistent versions across projects. Inspired by `git describe --tags`, `doxxer` offers tag filtering, custom version formatting via templates, and diverse version bumping strategies.

![image info](./docs/demo.gif)

# Introduction

## Getting Started

Install `doxxer` via `cargo` or by downloading a binary from the [GitHub Releases](https://github.com/karlis-vagalis/doxxer/releases) page.

**Cargo:**
```bash
cargo install doxxer
```

## Configuration

`doxxer` loads settings in layers, with later layers overriding earlier ones:
1. Default values
2. Config files in Current Working Directory (`.doxxer.toml`, then `doxxer.toml`)
3. Specified config file (`--config <file>`) or files in a specified directory (`--config <dir>` or `--directory <dir>`)
4. Environment variables (e.g., `DOXXER__OUTPUT__FORMAT=json`)
5. Command-line arguments (e.g., `--output json`)

For full details on configuration loading and precedence, see [Advanced Configuration](./docs/ADVANCED_CONFIGURATION.md).

### Configuration Files

Configuration can be done via `doxxer.toml` or `.doxxer.toml` files using TOML syntax. These can define global settings or command-specific ones.
Example:
```toml
# Global setting
tag_filter = "^v[0-9]"

[output]
format = "json"

[next.major]
increment = 2
```
For detailed structure and file loading logic, refer to the [Advanced Configuration](./docs/ADVANCED_CONFIGURATION.md).

### Environment Variables

Set environment variables prefixed with `DOXXER__`, using `__` for nesting (e.g., `DOXXER__OUTPUT__FORMAT=json`, `DOXXER__NEXT__MAJOR__INCREMENT=2`). See the [Advanced Configuration](./docs/ADVANCED_CONFIGURATION.md) for a comprehensive list of variables mapped to config options.

## Usage

`doxxer` is controlled via two main subcommands: `current` and `next`.
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
You can extract a specific field of the version (e.g., `major`, `minor`) using the `-F, --field` option. For detailed command options, see [Advanced Configuration](./docs/ADVANCED_CONFIGURATION.md) or run `doxxer current --help`.

### Next Version
The `next` command calculates the next SemVer version based on a chosen strategy.
```bash
doxxer next [STRATEGY]
```
If no strategy is specified, it defaults to `dev`. You can use `-F, --field` to extract specific parts of the version. Each strategy (e.g., `major`, `patch`, `prerelease`) has specific options. For detailed information on strategies and their options, consult the [Advanced Configuration](./docs/ADVANCED_CONFIGURATION.md) or run `doxxer next --help`.

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

For a comprehensive list of variables, their detailed descriptions, and examples, please see the [Template Variables Details](./docs/ADVANCED_CONFIGURATION.md#template-variables-details) in our advanced guide.

## Usage Examples

This section provides practical examples of how to use `doxxer` with different configurations.
(Note: `<hash>` in outputs refers to a short commit hash like `a1b2c3d`.)

### 1. Using a `doxxer.toml` Configuration File

Create a `doxxer.toml` file in your project's root directory:

```toml
# Global settings
directory = "."
tag_filter = "^v(\\d+\\.\\d+\\.\\d+)$" # Match tags like v1.0.0, capturing the SemVer part

[output]
format = "json" # Always output in JSON format
template = "version: {version}" # Custom output string

[next.patch]
# Specific settings for 'doxxer next patch'
increment = 5
prerelease.identifier = "rc"
prerelease.template = "{identifier}.{inc}" # e.g. rc.1
build_metadata.template = "stable.{hash}"

[next.dev]
# Specific settings for 'doxxer next dev'
prerelease.identifier = "dev" # Optional, "dev" is the default for dev strategy
prerelease.template = "{identifier}.{distance}.{hash}" # e.g. dev.3.a1b2c3d
build_metadata.template = "" # No build metadata for this dev version format
```

**With this `doxxer.toml` in place (assuming latest tag matching filter is `v1.2.3`):**

*   `doxxer current`:
    Outputs current version (1.2.3) in JSON with the custom template.
    ```json
    {
      "version": "version: 1.2.3"
    }
    ```

*   `doxxer next patch`:
    If current is `1.2.3`, calculates `1.2.8-rc.1+stable.<hash>`. Output is JSON.
    ```json
    {
      "version": "version: 1.2.8-rc.1+stable.<hash>"
    }
    ```
    *(Explanation: Patch increments by 5 from `[next.patch]`. Prerelease uses "rc" identifier and "{identifier}.{inc}" template. Build metadata uses "stable.{hash}" template.)*

*   `doxxer next minor`:
    If current is `1.2.3`, calculates `1.3.0-build.1`. Output is JSON.
    ```json
    {
      "version": "version: 1.3.0-build.1"
    }
    ```
    *(Explanation: No specific `[next.minor]` config. Uses default increment 1. Default prerelease identifier for non-dev strategies is "build". Default prerelease template is "{identifier}.{inc}". Default build metadata is empty.)*

*   `doxxer next dev` (assuming 3 commits since `v1.2.3`):
    If current is `1.2.3`, calculates `1.2.3-dev.3.<hash>`. Output is JSON.
    ```json
    {
      "version": "version: 1.2.3-dev.3.<hash>"
    }
    ```
    *(Explanation: Uses settings from `[next.dev]`. `prerelease.template` is "{identifier}.{distance}.{hash}". `build_metadata.template` is explicitly empty.)*

### 2. Overriding Settings with CLI Arguments

CLI arguments take precedence over `doxxer.toml` settings.

*   **Override output format to plain text (using `doxxer.toml` from Example 1):**
    ```bash
    doxxer current -f plain
    ```
    Output: `version: 1.2.3`

*   **Override increment for `next patch` (using `doxxer.toml` from Example 1):**
    If current is `1.2.3`, `doxxer.toml` would make `next patch` result in `1.2.8-rc.1+stable.<hash>`.
    ```bash
    doxxer next patch --increment 1
    ```
    This command would result in `1.2.4-rc.1+stable.<hash>` (patch version is `1.2.3` + 1 = `1.2.4`). Output in JSON as per `doxxer.toml`.

*   **Specify a full prerelease version for `next major` (current `1.2.3`):**
    This creates a "premajor" version.
    ```bash
    doxxer next premajor --identifier alpha --prerelease-template "{identifier}.{inc}.{hash}"
    ```
    This might give `2.0.0-alpha.1.<hash>`. Output in plain text by default if no config file sets otherwise.

### 3. Using Environment Variables

Configure `doxxer` without a config file, using environment variables.

*   **Set global output format and `next major` increment:**
    ```bash
    export DOXXER__OUTPUT__FORMAT=json
    export DOXXER__NEXT__MAJOR__INCREMENT=2
    # Assuming latest tag is v1.2.3
    doxxer next major
    ```
    This would calculate `3.0.0` (major +2, minor/patch reset) and output it in JSON format.
    ```json
    {
      "major": 3,
      "minor": 0,
      "patch": 0,
      "full": "3.0.0"
    }
    ```
    *(Note: Default output template is `{version}`. If `DOXXER__OUTPUT__TEMPLATE` was also set, it would be used.)*

*   **Set a specific prerelease identifier and template for the `dev` strategy:**
    ```bash
    export DOXXER__NEXT__DEV__PRERELEASE_IDENTIFIER="snapshot"
    export DOXXER__NEXT__DEV__PRERELEASE_TEMPLATE="{identifier}.{distance}"
    # Assuming latest tag v1.2.3, 5 commits since tag
    doxxer next dev
    ```
    This would result in `1.2.3-snapshot.5+{hash}` (default build metadata for dev is `{hash}`).
    To also customize build metadata: `export DOXXER__NEXT__DEV__BUILD_METADATA_TEMPLATE="build.{hash}"`

### 4. Common Use Cases & Specific Scenarios

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
    Output (assuming current version 1.2.3): `1`

*   **Generate `next patch` version relying on defaults:**
    If no `doxxer.toml` exists or it has no `[next.patch]` section:
    ```bash
    # Assuming latest tag v1.2.3
    doxxer next patch
    ```
    Output (plain text): `1.2.4`
    *(Default increment is 1. Default prerelease/build for plain patch bump are empty.)*

*   **Customizing prerelease identifier for multiple strategies (e.g., for release candidates):**
    To use "rc" for all prerelease types except `dev`:
    ```toml
    # In doxxer.toml
    [next.prerelease]
    identifier = "rc"
    # prerelease_template = "{identifier}.{inc}" # This is the default

    [next.prepatch]
    identifier = "rc"

    [next.preminor]
    identifier = "rc"

    [next.premajor]
    identifier = "rc"
    ```
    This ensures `doxxer next prepatch`, `doxxer next preminor`, etc., will use `rc.1`, `rc.2`, etc.

*   **Generate `dev` versions like `1.2.3-dev.5+commit.abcdef`:**
    *   In `doxxer.toml`:
        ```toml
        [next.dev]
        # identifier = "dev" # This is the default for dev strategy
        prerelease_template = "dev.{distance}"
        build_metadata_template = "commit.{hash}"
        ```
    *   Then run: `doxxer next dev` (assuming current `1.2.3`, 5 commits since tag)
    *   Output: `1.2.3-dev.5+commit.<hash>`

### 5. Using a `.doxxer.toml` for Project Defaults

If you have a `.doxxer.toml` in your project's root (current working directory):
```toml
# .doxxer.toml
[output]
template = "ProjX-{version}"

[next.patch]
increment = 2
prerelease.identifier = "alpha"
```
And you run `doxxer next patch` (assuming current `1.0.0`):
*   It will calculate `1.0.2-alpha.1`.
*   The output will be `ProjX-1.0.2-alpha.1` (plain text by default).

If you then have a specific `doxxer.toml` (e.g., for CI) in a subdirectory `conf/ci.toml`:
```toml
# conf/ci.toml
[output]
format = "json"
template = "{version}" # Override project's output template

[next.patch]
# Inherits increment = 2 from .doxxer.toml if not overridden
prerelease.identifier = "rc" # Override prerelease identifier
```
Running `doxxer --config conf/ci.toml next patch` (current `1.0.0`):
*   It will calculate `1.0.2-rc.1`. (Increment 2 from `.doxxer.toml`, 'rc' from `ci.toml`).
*   Output will be JSON: `{"full": "1.0.2-rc.1", ...}` (template from `ci.toml`).
This demonstrates the layering of CWD config and specified config.

## Docker Support

There is a docker image based on latest `alpine` image and published on GitHub's containter registry: `ghcr.io/karlis-vagalis/doxxer`

### Settings

The default workspace directory inside the container is `/repo`.

The entrypoint is `doxxer`.

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
    A: This behavior is intentional for the `dev` strategy. Its default prerelease template is `"{pre}.{identifier}.{distance}"`. The `{pre}` variable captures the *entire existing* prerelease string. This design allows you to see the lineage from a previous prerelease tag (like `alpha.1`) while also appending the `dev` specific information (identifier and commit distance). It's aimed at providing maximum context during development. If you prefer a different format for `dev` versions, you can customize `next.dev.prerelease_template` and `next.dev.identifier` in your configuration.

2.  **Q: Why is the project called `doxxer`?**
    A: Because it's like somebody who [doxes](https://en.wikipedia.org/wiki/Doxing), by exposing the version information about Git repo to you! Is the name really fitting? Maybe not. It's just a name.

3.  **Q: Why Rust?**
    A: To learn the language, and Rust offers excellent support for creating binary CLI tools, particularly with libraries like `clap` for argument parsing.

4.  **Q: How do I set an option globally for all commands?**
    A: Global options (like `directory`, `output.format`, `filter.tag`) can be set at the top level of your `doxxer.toml` file or within general tables like `[output]` and `[filter]`. These apply unless overridden by more specific configurations (command-specific, environment variables, or CLI arguments). For environment variables, use the base prefixes (e.g., `DOXXER__OUTPUT__FORMAT`, `DOXXER__FILTER__TAG`). Refer to the "Configuration Priority" and "Configuration Files" sections for more details.

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
    [next.prepatch]
    identifier = "rc" # e.g., for release candidates on patches, version like 1.2.3-rc.1

    [next.preminor]
    identifier = "beta" # e.g., for beta releases on minor bumps, version like 1.3.0-beta.1
    ```
    The same principle applies to other strategy-specific settings like `increment` values or custom `prerelease_template` and `build_metadata_template`. Refer to the "Configuration Options Table" for all available configuration keys.

7.  **Q: Why does my `tag_filter` not seem to work?**
    A: Common reasons include:
    *   **Regex Syntax:** Ensure your regular expression is valid (e.g., correctly escaped in TOML) and accurately describes the tags you want `doxxer` to consider. Test your regex with a dedicated tool if unsure.
    *   **Matching Scope:** The regex usually needs to match the entire part of the tag name you intend to parse for versioning. For example, to match tags like `v1.0.0`, `v1.2.3`, a regex like `^v(\d+\.\d+\.\d+)$` is appropriate if you want to capture the SemVer part.
    *   **Configuration Priority:** Double-check that your intended `tag_filter` isn't being overridden by a higher-priority source (like a CLI argument or an environment variable). See "Configuration Priority".
    *   **No Matching Tags:** There might genuinely be no tags in your repository that match the filter and also contain a parsable SemVer string.
    *   **Interaction with SemVer Parsing:** `doxxer` first filters tags using `tag_filter`, then attempts to parse a SemVer string from the *matching tag name*. If your filter matches a tag that isn't itself a valid SemVer string (or doesn't start with one, possibly after stripping a 'v'), it will be ignored.

8.  **Q: How does `tag_filter` interact with SemVer parsing?**
    A: `doxxer` first applies the `tag_filter` regex to all Git tag names. For each tag name that matches the filter, `doxxer` then attempts to find and parse a Semantic Version from that tag name. It can handle common prefixes like 'v' automatically (e.g., `v1.2.3` is parsed as `1.2.3`). If your `tag_filter` matches a tag name from which a valid SemVer string cannot be extracted, that tag will be ignored when determining the latest version. For example, if `tag_filter = "release-(.*)"` matches `release-my-app-1.2.3`, doxxer will attempt to parse `my-app-1.2.3` as SemVer, which would likely fail unless the filter was more specific like `release-my-app-(v?\d+\.\d+\.\d+)`. It's best if your filter is designed to match tags that are clearly SemVer compatible.

9.  **Q: Why do strategies like `premajor` or `prerelease` default to `build.1`-style prereleases, and `dev` to a different style?**
    A: `doxxer` has distinct default behaviors for "release-oriented" prereleases versus "development" prereleases:
    *   For strategies like `prerelease`, `premajor`, `preminor`, and `prepatch`, the default prerelease identifier is `"build"` and the default template is `"{identifier}.{inc}"` (e.g., `build.1`, `build.2`). This provides a generic prerelease sequence.
    *   For the `dev` strategy (which is also the default for `doxxer next` if no strategy is given), the default identifier is `"dev"` and the template is `"{pre}.{identifier}.{distance}"` (e.g., `dev.5` or `alpha.1.dev.5`). This template is designed to append to existing prereleases and include commit distance, offering more context during active development.
    These defaults can all be overridden in the configuration if you prefer a different style for any strategy (see the "Configuration Options Table"). For instance, a simple `doxxer next patch` (without `pre`) will not add any prerelease by default.

10. **Q: My Git tag is `v1.2.3`, but `doxxer current` outputs `1.2.3`. Where did the 'v' go?**
    A: `doxxer` treats the 'v' prefix on tags (like `v1.2.3`) as a common convention but not part of the core SemVer version itself. It automatically strips this 'v' when parsing the tag to determine the semantic version. The output (`{version}` variable) will be the pure `MAJOR.MINOR.PATCH` and any prerelease/build metadata. If you need a 'v' prefix in the final output string, you should use the `output.template` option, for example: `doxxer current -o "v{version}"` or by setting `output.template = "v{version}"` in your config file.

11. **Q: How are multiple prerelease identifiers handled if the current version already has one and I use `next dev`?**
    A: The `dev` strategy's default prerelease template is `"{pre}.{identifier}.{distance}"`. The `{pre}` variable is populated with the *entire existing prerelease string* of the current version.
    For example, if your current tagged version is `1.2.3-alpha.1` and you run `doxxer next dev` (assuming the new identifier is `dev` and commit `distance` is 5):
    - `{pre}` will be `alpha.1`.
    - `{identifier}` will be `dev`.
    - `{distance}` will be `5`.
    The resulting prerelease string will be `alpha.1.dev.5`, making the full version `1.2.3-alpha.1.dev.5`.
    Other strategies (like `major`, `minor`, `patch`, `prerelease`, `premajor`, etc.) typically replace the existing prerelease segment entirely or increment parts of it, rather than appending new identifiers after the full existing segment in this way. You can, of course, customize the `prerelease_template` for any strategy to achieve different concatenation effects if needed.

# Roadmap

- [ ] Add installation shell script, similar to `uv` or `just` to install prebuild binaries from GitHub
- [ ] Add `{timestamp}` variable support to the template