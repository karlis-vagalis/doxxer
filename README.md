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

`doxxer` offers a flexible configuration system, allowing you to tailor its behavior to your specific needs. Settings can be applied through command-line arguments, environment variables, configuration files, or rely on sensible defaults.

### Configuration Priority

Settings are applied in the following order of precedence (highest to lowest):

1.  **Command-line Arguments:** Options passed directly when running `doxxer` (e.g., `--output json`). These override all other settings.
2.  **Environment Variables:** Variables prefixed with `DOXXER_` (e.g., `DOXXER_OUTPUT__FORMAT=json`).
3.  **Configuration Files:** Settings defined in `doxxer.toml` or `.doxxer.toml` files.
4.  **Default Values:** Built-in defaults if no other configuration is provided.

### Configuration Files

`doxxer` can be configured using a TOML file named `doxxer.toml` or `.doxxer.toml`.

**Search Order:**

*   If a path is provided via the `-c, --config <PATH>` CLI option:
    *   If `<PATH>` is a file, it's loaded directly.
    *   If `<PATH>` is a directory, `doxxer` looks for `.doxxer.toml` then `doxxer.toml` within that directory.
*   If no `--config` path is given:
    *   If a `--directory <PATH>` (for the Git repo) is provided, `doxxer` looks in that directory first (for `.doxxer.toml` then `doxxer.toml`).
    *   Finally, it looks in the current working directory (for `.doxxer.toml` then `doxxer.toml`).

**Structure:**

Configuration settings can be global or command-specific.

*   **Global Settings:** Apply to all commands unless overridden by a more specific setting.
    ```toml
    # Example global settings in doxxer.toml
    directory = "/path/to/your/git/repo" # Default repository path
    tag_filter = "^v[0-9]"              # Filter tags starting with 'v' and a digit

    [output]                            # Settings related to output
    format = "json"                     # Default output format
    template = "{version}-final"        # Default output template
    ```

*   **Command-Specific Settings:** Apply only to a particular command (e.g., `current`, `next`) or even a specific strategy within `next` (e.g., `next.major`, `next.patch`).
    ```toml
    # Example command-specific settings in doxxer.toml

    [current]
    # Override output format specifically for the 'current' command
    output.format = "plain"

    [next]
    # Default settings for all 'next' strategies
    prerelease.identifier = "beta"

    [next.major]
    # Settings specific to 'doxxer next major'
    increment = 2 # Increment major version by 2
    build_metadata.template = "prod.{hash}"

    [next.patch]
    # Settings specific to 'doxxer next patch'
    prerelease.identifier = "rc"
    ```

### Environment Variables

You can configure `doxxer` using environment variables.

*   **Prefix:** All variables must start with `DOXXER_`.
*   **Structure:** For nested settings (like those in TOML tables or for specific commands/strategies), use a double underscore `__` as a separator.

**Examples:**

*   Set the global output format:
    `export DOXXER_OUTPUT__FORMAT=json`
*   Set the increment for the `next major` command:
    `export DOXXER_NEXT__MAJOR__INCREMENT=2`
*   Set the prerelease identifier for the `next prerelease` command:
    `export DOXXER_NEXT__PRERELEASE__IDENTIFIER=alpha`
*   Set the global tag filter:
    `export DOXXER_TAG_FILTER="^v"`

### Configuration Options Table

The following table summarizes all available configuration options, their corresponding CLI flags, configuration file keys, environment variables, and default values.

| Purpose                                    | Option Name                  | CLI Flag(s)                     | Config File Key                      | Environment Variable                      | Default Value                                  |
| ------------------------------------------ | ---------------------------- | ------------------------------- | ------------------------------------ | ----------------------------------------- | ---------------------------------------------- |
| **Global Options**                         |                              |                                 |                                      |                                           |                                                |
| Path to the Git repository                 | `directory`                  | `-d, --directory <PATH>`        | `directory`                          | `DOXXER_DIRECTORY`                        | `.` (current directory)                        |
| Path to config file or directory           | `config`                     | `-c, --config <PATH>`           | N/A (CLI only)                       | N/A (CLI only)                            | (none)                                         |
| Regex to filter relevant Git tags          | `filter.tag`                 | `-t, --tag-filter <REGEX>`      | `filter.tag`                         | `DOXXER_FILTER__TAG`                      | `""` (empty string, no filter)                 |
| Output format for the version              | `output.format`              | `-f, --format <FORMAT>`         | `output.format`                      | `DOXXER_OUTPUT__FORMAT`                   | `plain`                                        |
| Template for the resulting version string  | `output.template`            | `-o, --template <TEMPLATE>`     | `output.template`                    | `DOXXER_OUTPUT__TEMPLATE`                 | `{version}`                                    |
| **Command: `current` & `next`**            |                              |                                 |                                      |                                           |                                                |
| Field/part of the version to output        | `field`                      | `-F, --field <FIELD>`           | N/A (CLI only)                       | N/A (CLI only)                            | (outputs full version)                         |
| **Command: `next` (General)**              |                              |                                 | `[next]`                             | `DOXXER_NEXT`                             |                                                |
| Default prerelease identifier              | `prerelease.identifier`      |                                 | `next.prerelease.identifier`         | `DOXXER_NEXT__PRERELEASE__IDENTIFIER`     | `build`                                        |
| Default prerelease template                | `prerelease.template`        |                                 | `next.prerelease.template`           | `DOXXER_NEXT__PRERELEASE__TEMPLATE`       | `{identifier}.{inc}`                           |
| Default build metadata template            | `build_metadata.template`    |                                 | `next.build_metadata.template`       | `DOXXER_NEXT__BUILD_METADATA__TEMPLATE`   | `""` (empty string)                            |
| **Strategy: `major`, `minor`, `patch`**    |                              |                                 | `[next.major]`, etc.                 | `DOXXER_NEXT__MAJOR`, etc.                |                                                |
| Version bump increment                     | `increment`                  | `--increment <NUM>`             | `next.<strategy>.increment`          | `DOXXER_NEXT__<STRATEGY>__INCREMENT`      | `1`                                            |
| Build metadata template                    | `build_metadata.template`    | `--build-metadata-template <TPL>` | `next.<strategy>.build_metadata.template` | `DOXXER_NEXT__<STRATEGY>__BUILD_METADATA__TEMPLATE` | `""` (empty string)                            |
| **Strategy: `prerelease`**                 |                              |                                 | `[next.prerelease]`                  | `DOXXER_NEXT__PRERELEASE`                 |                                                |
| Prerelease identifier                      | `identifier`                 | `<IDENTIFIER>` (positional)     | `next.prerelease.identifier`         | `DOXXER_NEXT__PRERELEASE__IDENTIFIER`     | `build`                                        |
| Template for prerelease part               | `prerelease_template`        | `--prerelease-template <TPL>`   | `next.prerelease.template`           | `DOXXER_NEXT__PRERELEASE__PRERELEASE_TEMPLATE` | `{identifier}.{inc}`                           |
| Build metadata template                    | `build_metadata.template`    | `--build-metadata-template <TPL>` | `next.prerelease.build_metadata.template` | `DOXXER_NEXT__PRERELEASE__BUILD_METADATA__TEMPLATE` | `""` (empty string)                            |
| **Strategy: `premajor`, `preminor`, `prepatch`** |                        |                                 | `[next.premajor]`, etc.              | `DOXXER_NEXT__PREMAJOR`, etc.             |                                                |
| Version bump increment                     | `increment`                  | `--increment <NUM>`             | `next.<strategy>.increment`          | `DOXXER_NEXT__<STRATEGY>__INCREMENT`      | `1`                                            |
| Prerelease identifier                      | `identifier`                 | `<IDENTIFIER>` (positional)     | `next.<strategy>.identifier`         | `DOXXER_NEXT__<STRATEGY>__IDENTIFIER`     | `build`                                        |
| Template for prerelease part               | `prerelease_template`        | `--prerelease-template <TPL>`   | `next.<strategy>.prerelease_template`| `DOXXER_NEXT__<STRATEGY>__PRERELEASE_TEMPLATE` | `{identifier}.{inc}`                           |
| Build metadata template                    | `build_metadata.template`    | `--build-metadata-template <TPL>` | `next.<strategy>.build_metadata.template` | `DOXXER_NEXT__<STRATEGY>__BUILD_METADATA__TEMPLATE` | `""` (empty string)                            |
| **Strategy: `dev`**                        |                              |                                 | `[next.dev]`                         | `DOXXER_NEXT__DEV`                        |                                                |
| Prerelease identifier                      | `identifier`                 | `<IDENTIFIER>` (positional)     | `next.dev.identifier`                | `DOXXER_NEXT__DEV__IDENTIFIER`            | `dev`                                          |
| Template for prerelease part               | `prerelease_template`        | `--prerelease-template <TPL>`   | `next.dev.template`                  | `DOXXER_NEXT__DEV__PRERELEASE_TEMPLATE`   | `{pre}.{identifier}.{distance}`                |
| Build metadata template                    | `build_metadata.template`    | `--build-metadata-template <TPL>` | `next.dev.build_metadata.template`   | `DOXXER_NEXT__DEV__BUILD_METADATA__TEMPLATE` | `{hash}`                                       |

## Usage

`doxxer` is controlled via two main subcommands: `current` and `next`. Several global options can be applied to modify behavior across commands.

**Global CLI Options:**

The following global options can be used with `doxxer`:

*   `-d, --directory <PATH>`: Path to the Git repository.
*   `-c, --config <PATH>`: Path to a specific configuration file or directory containing `doxxer.toml`.
*   `-t, --tag-filter <REGEX>`: A regular expression to filter Git tags.
*   `-f, --format <FORMAT>`: Specifies the output format (`plain`, `json`).
*   `-o, --template <TEMPLATE>`: Defines the output template string.

For detailed information on how these options are configured via files or environment variables, and their default values, please see the main [Configuration](#configuration) section.

The general syntax is:
```bash
doxxer [GLOBAL OPTIONS] <COMMAND> [COMMAND OPTIONS]
```

You can always get help with:
```bash
doxxer help
```
Which outputs:
```text
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

### Current Version

The `current` command retrieves the latest SemVer version from your Git tags.
```bash
doxxer current [OPTIONS]
```
You can specify a particular field of the version to extract using the `-F, --field` option (e.g., `major`, `minor`, `patch`, `prerelease`, `build-metadata`).

For detailed help:
```bash
doxxer current --help
```
Result:
```text
Get current version

Usage: doxxer current [OPTIONS]

Options:
  -F, --field <FIELD>  Field/part of the version [possible values: major, minor, patch, prerelease, build-metadata]
  -h, --help           Print help
```

### Next Version

The `next` command calculates the next SemVer version based on a chosen strategy and the current state of the repository.
```bash
doxxer next [OPTIONS] [STRATEGY] [STRATEGY_OPTIONS]
```
If no strategy is specified, `doxxer next` defaults to the `dev` strategy, which is ideal for generating development versions.

Like the `current` command, you can use the `-F, --field` option to extract a specific part of the calculated next version.

Each strategy (e.g., `major`, `minor`, `patch`, `prerelease`, `dev`) has its own set of options (like `--increment`, `--identifier`, `--prerelease-template`). These, along with their configuration file and environment variable counterparts, are detailed in the [Configuration Options Table](#configuration-options-table).

For detailed help on the `next` command and its strategies:
```bash
doxxer next --help
```
Result:
```text
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
  -F, --field <FIELD>  Field/part of the version [possible values: major, minor, patch, prerelease, build-metadata]
  -h, --help           Print help
```

## Template Variables

`doxxer` uses templates for formatting the final version string (`output.template`), the prerelease segment (`prerelease.template`), and the build metadata segment (`build_metadata.template`). These templates can use several variables that will be replaced with dynamic values.

### For `output.template`

This template is used to construct the final version string that `doxxer` outputs.

| Variable    | Description                                                                 | Example Default Usage |
| :---------- | :-------------------------------------------------------------------------- | :-------------------- |
| `{version}` | The full SemVer version string (e.g., `1.2.3`, `1.2.3-rc.1`, `1.2.3+build.456`). **Required.** | `"{version}"`         |

**Example `output.template` values:**
*   `"v{version}"` -> `v1.2.3`
*   `"{version}-release"` -> `1.2.3-rc.1-release`

### For `prerelease.template`

This template defines the format of the prerelease part of the version (e.g., the `rc.1` in `1.2.3-rc.1`). It is used when a prerelease version is being generated.

| Variable       | Description                                                                                                | Example Default Usage                               |
| :------------- | :--------------------------------------------------------------------------------------------------------- | :-------------------------------------------------- |
| `{pre}`        | The existing prerelease part of the current version, if any. Useful for additive prereleases like in the `dev` strategy. | `"{pre}.{identifier}.{distance}"` (for `dev`)        |
| `{identifier}` | The identifier for the prerelease (e.g., `alpha`, `beta`, `rc`, `dev`). This comes from the `identifier` setting. | `"{identifier}.{inc}"` (standard prerelease)        |
| `{inc}`        | The auto-incrementing number for the current prerelease identifier.                                        | `"{identifier}.{inc}"`                              |
| `{distance}`   | The number of commits since the last tag. Often used in `dev` versions.                                    | `"{pre}.{identifier}.{distance}"` (for `dev`)        |
| `{hash}`       | The first 7 characters of the current commit hash.                                                         | (More common in build metadata but can be used here) |

**Example `prerelease.template` values (assuming current version `1.2.0` and identifier `rc`):**
*   `"{identifier}.{inc}"` (default) -> `rc.1` (next version might be `1.2.1-rc.1` or `1.2.0-rc.1` depending on strategy)
*   `"preview.{inc}"` -> `preview.1`
*   If current is `1.2.0-alpha.1`, strategy is `dev` (identifier `dev`), `distance` is 5: `"{pre}.{identifier}.{distance}"` -> `alpha.1.dev.5` (making the version `1.2.0-alpha.1.dev.5`)


### For `build_metadata.template`

This template defines the format of the build metadata part of the version (e.g., the `build.456` in `1.2.3+build.456`).

| Variable     | Description                                                              | Example Default Usage          |
| :----------- | :----------------------------------------------------------------------- | :----------------------------- |
| `{hash}`     | The first 7 characters of the current commit hash.                       | `"{hash}"` (for `dev` strategy) |
| `{distance}` | The number of commits since the last tag.                                |                                |
| `{pre}`      | The existing prerelease part of the current version, if any.             |                                |
| `{identifier}` | The prerelease identifier, if a prerelease is part of the main version.  |                                |
| `{inc}`      | The prerelease increment number, if a prerelease is part of the main version. |                                |


**Example `build_metadata.template` values:**
*   `"build.{hash}"` -> `build.a1b2c3d`
*   `"commit.{distance}.{hash}"` -> `commit.10.a1b2c3d`

**Note:** The availability and exact value of variables like `{pre}`, `{identifier}`, and `{inc}` in the `build_metadata.template` can depend on whether the version *before* adding build metadata already contains a prerelease segment. The `{hash}` and `{distance}` are generally always available.

## Usage Examples

This section provides practical examples of how to use `doxxer` with different configurations.

### 1. Using a `doxxer.toml` Configuration File

Create a `doxxer.toml` file in your project's root directory:

```toml
# Global settings
directory = "."
tag_filter = "^v\d+\.\d+\.\d+$" # Match tags like v1.0.0

[output]
format = "json" # Always output in JSON format
template = "version: {version}"

[next]
# Default prerelease identifier for all 'next' commands that generate prereleases
prerelease.identifier = "beta"
prerelease.template = "{identifier}.{inc}" # e.g., beta.1

[next.patch]
# Specific settings for 'doxxer next patch'
increment = 5 # Increment patch version by 5
prerelease.identifier = "rc" # Release candidates for patches
build_metadata.template = "stable.{hash}"

[next.dev]
prerelease.template = "dev.{distance}.{hash}" # Custom dev version format
```

**With this `doxxer.toml` in place:**

*   `doxxer current`: Outputs current version in JSON, like `{"version": "version: v1.2.3"}` (assuming `v1.2.3` is the latest tag matching the filter).
*   `doxxer next patch`: If current is `v1.2.3`, calculates `v1.2.8-rc.1+stable.<hash>` (patch increments by 5, uses 'rc' identifier). Output is JSON.
*   `doxxer next minor`: If current is `v1.2.3`, calculates `v1.3.0-beta.1`. Output is JSON. (Uses default `next.prerelease.identifier` 'beta').
*   `doxxer next dev`: If current is `v1.2.3` and there are 3 commits since the tag, calculates `v1.2.3-dev.3.<hash>`. Output is JSON.

### 2. Overriding Settings with CLI Arguments

Even with a `doxxer.toml`, CLI arguments take precedence.

*   **Override output format to plain text:**
    ```bash
    doxxer current -f plain
    ```
    *(Output: `version: v1.2.3`)*

*   **Override increment for `next patch` for this run only:**
    ```bash
    doxxer next patch --increment 1
    ```
    *(If current is `v1.2.3` and `doxxer.toml` from above is used, this would result in `v1.2.4-rc.1+stable.<hash>` instead of `v1.2.8-rc.1+stable.<hash>`)*

*   **Specify a different prerelease identifier for `next major`:**
    ```bash
    doxxer next major --identifier alpha --prerelease-template "{identifier}.{inc}.{hash}"
    ```
    *(If current is `v1.2.3`, this might give `v2.0.0-alpha.1.<hash>`)*

### 3. Using Environment Variables

Configure `doxxer` without a config file, using environment variables.

*   **Set global output format and next major increment:**
    ```bash
    export DOXXER_OUTPUT__FORMAT=json
    export DOXXER_NEXT__MAJOR__INCREMENT=2
    doxxer next major
    ```
    *(If current is `v1.2.3`, this would calculate `v3.0.0` and output it in JSON format.)*

*   **Set a specific prerelease identifier and template for the `dev` strategy:**
    ```bash
    export DOXXER_NEXT__DEV__IDENTIFIER="snapshot"
    export DOXXER_NEXT__DEV__PRERELEASE_TEMPLATE="{identifier}.{timestamp}" # Assuming {timestamp} was a feature
    # Note: {timestamp} is not a real variable. This is for illustration of env var usage.
    # Use a real template like: export DOXXER_NEXT__DEV__PRERELEASE_TEMPLATE="{identifier}.{distance}"
    doxxer next dev
    ```

### 4. Common Use Cases

*   **Always output JSON:**
    *   In `doxxer.toml`:
        ```toml
        [output]
        format = "json"
        ```
    *   Or with env var: `export DOXXER_OUTPUT__FORMAT=json`
    *   Or CLI: `doxxer <command> --format json`

*   **Set a global prerelease identifier (e.g., "ga", "prod") for non-dev bumps:**
    *   In `doxxer.toml`:
        ```toml
        [next.prerelease] # Affects 'prerelease' strategy directly
        identifier = "ga"

        [next.premajor]   # Affects 'premajor' strategy
        identifier = "ga"

        [next.preminor]
        identifier = "ga"

        [next.prepatch]
        identifier = "ga"
        ```
    *   *Note: You might need to set it for each strategy if you want a truly global override for all bump-plus-prerelease types, as `next.prerelease.identifier` from the general `[next]` section is a fallback.*

*   **Generate `dev` versions like `1.2.3-dev.5+commit.abcdef`:**
    *   In `doxxer.toml`:
        ```toml
        [next.dev]
        prerelease_template = "dev.{distance}"
        build_metadata_template = "commit.{hash}"
        ```
    *   Then run: `doxxer next dev`

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

4.  **Q: How do I set an option globally for all commands?**
    A: You can set global options in your `doxxer.toml` file outside of any specific command table (e.g., general settings at the top, or within `[output]`, `[filter]` which apply globally unless overridden by command-specific sections). For environment variables, use the base prefixes (e.g., `DOXXER_OUTPUT__FORMAT`). CLI arguments are inherently command-specific if they are not one of the global flags like `-d` or `-f`. Refer to the [Configuration Priority](#configuration-priority) and [Configuration Files](#configuration-files) sections for more details.

5.  **Q: What's the easiest way to always output JSON (or another format)?**
    A: The most persistent way is to set it in your `doxxer.toml` file:
    ```toml
    [output]
    format = "json"
    ```
    Alternatively, you can export an environment variable: `export DOXXER_OUTPUT__FORMAT=json`. If you only need it for a single command execution, use the `-f json` (or `--format json`) CLI flag.

6.  **Q: Can I have different prerelease identifiers for `patch` releases versus `minor` releases when using strategies like `prepatch` or `preminor`?**
    A: Yes! You can define this in your `doxxer.toml` by targeting the specific strategy:
    ```toml
    [next.prepatch]
    identifier = "rc" # e.g., for release candidates on patches, version like 1.2.3-rc.1

    [next.preminor]
    identifier = "beta" # e.g., for beta releases on minor bumps, version like 1.2.0-beta.1
    ```
    The same principle applies to other strategy-specific settings like `increment` values or custom `prerelease_template` and `build_metadata_template`. Refer to the [Configuration Options Table](#configuration-options-table) for all available configuration keys.

7.  **Q: Why does my `tag_filter` not seem to work?**
    A: Common reasons include:
    *   **Regex Syntax:** Ensure your regular expression is valid and accurately describes the tags you want `doxxer` to consider.
    *   **Matching:** By default, the regex needs to match the entire tag name. For example, to match tags like `v1.0.0`, `v1.2.3`, a regex like `^v\d+\.\d+\.\d+$` is appropriate (note TOML string escaping for backslashes: `^v\\d+\\.\\d+\\.\\d+$`).
    *   **Configuration Priority:** Double-check that your intended `tag_filter` isn't being overridden by a higher-priority source (like a CLI argument or an environment variable). See [Configuration Priority](#configuration-priority).
    *   **No Matching Tags:** There might genuinely be no tags in your repository that match the filter.
    Test your regex with a dedicated regex testing tool if you're unsure about its correctness.

# Roadmap

- [ ] Add installation shell script, similar to `uv` or `just` to install prebuild binaries from GitHub
- [ ] Add `{timestamp}` variable support to the template
