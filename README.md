✨**doxxer**✨ is your secret weapon for taming software versions! This highly configurable CLI tool, written in *Rust*, automates dynamic [SemVer](https://semver.org/) versioning by using your latest Git tags and commit history.

No more manual versioning headaches and inconsistent version numbers scattered across different projects and programming languages. `doxxer` streamlines this process, ensuring your versions and release tags are always accurate and consistent.

Inspired by the utility of `git describe --tags`, it offers robust features like tag filtering, powerful templating for custom version formats, various version bumping strategies, and multiple output options.

![image info](./docs/demo.gif)

# Introduction

## Getting started

You can currently install the tool locally via *cargo*:

```bash
cargo install doxxer
```

or downloading latest binary from [GitHub Release](https://github.com/karlis-vagalis/doxxer/releases) page.

## Configuration

`doxxer` offers a flexible configuration system, allowing you to tailor its behavior to your specific needs. Settings can be applied through command-line arguments, environment variables, configuration files, or rely on sensible defaults.

### Configuration Priority

`doxxer` loads settings in layers. Each subsequent layer can override the settings defined in the previous ones. The order of loading and precedence (from lowest to highest) is as follows:

1.  **Default Values:** These are the built-in defaults for all settings. This is the base layer.
2.  **Configuration Files in Current Working Directory (CWD):** `doxxer` automatically loads `.doxxer.toml` and then `doxxer.toml` (if they exist) from the current working directory. Settings in `doxxer.toml` override those in `.doxxer.toml` if both are present and define the same key. These files act as a foundational layer of customization.
3.  **Specified Configuration File or Directory:**
    *   If a specific configuration file is provided via `--config <path_to_file>`, it's loaded, and its settings override those from the CWD files.
    *   If a directory is provided via `--config <path_to_dir>` or `--directory <path_to_repo_dir>` (and no specific file was given with `--config`), `doxxer` looks for `.doxxer.toml` then `doxxer.toml` within that directory. Found settings override the CWD layer. If both `--config <dir>` and `--directory <dir>` are provided, the path from `--config` is preferred.
4.  **Environment Variables:** Variables prefixed with `DOXXER_` (e.g., `DOXXER_OUTPUT__FORMAT=json`) are loaded next. These will override any settings from configuration files or defaults.
5.  **Command-line Arguments:** Options passed directly when running `doxxer` (e.g., `--output json` or `-f json`). These have the highest precedence and override all other settings.

Essentially, `doxxer` starts with defaults, then applies settings from CWD config files, then from a more specific config file/directory (if specified), then environment variables, and finally, CLI arguments make the ultimate decision.

### Configuration Files

`doxxer` can be configured using TOML files, typically named `doxxer.toml` or `.doxxer.toml`.

**Loading and Search Order:**

1.  **Current Working Directory (CWD) Scan:** `doxxer` always first attempts to load configuration from files named `.doxxer.toml` and then `doxxer.toml` located in the current working directory where `doxxer` is executed. If both exist, settings from `doxxer.toml` will take precedence over `.doxxer.toml` for any overlapping keys. These serve as a base configuration.
2.  **Specified Configuration (`--config <PATH>`):**
    *   If `<PATH>` is a **file** (e.g., `--config /path/to/myconfig.toml`), this specific file is loaded. Its settings override any found in the CWD configuration files.
    *   If `<PATH>` is a **directory** (e.g., `--config /path/to/confdir/`), `doxxer` looks for `.doxxer.toml` and then `doxxer.toml` within this directory. Settings from a found file (again, `doxxer.toml` preferred over `.doxxer.toml`) override the CWD configuration.
3.  **Repository Directory (`--directory <PATH>` as fallback for config):**
    *   If the `--config <PATH>` option is **not** used to specify a file, and a `--directory <PATH>` (for the Git repository) is provided and is different from the CWD, `doxxer` will search for `.doxxer.toml` and then `doxxer.toml` within this repository directory.
    *   Settings from a file found here will override the CWD configuration. This allows for project-specific configurations located with the repository itself, if not explicitly pointed to by `--config`.
    *   If `--config` specifies a directory, this step is effectively covered by point 2.

**Key Points for Config Files:**
*   Within a directory, `doxxer.toml` values take precedence over `.doxxer.toml` if both files exist and define the same settings.
*   Configuration files loaded via `--config` (either a direct file or files within a specified directory) take precedence over files found in the CWD or via `--directory`.
*   All loaded configurations are merged, with higher precedence sources (as listed in "Configuration Priority") overriding lower ones.

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

    # Example:
    # [next.minor]
    # prerelease.identifier = "beta" # Sets "beta" for "doxxer next minor" strategy

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

The following table summarizes all available configuration options, their corresponding CLI flags, configuration file keys (within `doxxer.toml` or `.doxxer.toml`), environment variables, and default values.

| Purpose                                    | Option Name (Conceptual)     | CLI Flag(s) / Arg              | Config File Key (`[table].key`)                | Environment Variable (`DOXXER_...`)             | Default Value                                     |
| ------------------------------------------ | ---------------------------- | ------------------------------- | ---------------------------------------------- | ----------------------------------------------- | ------------------------------------------------- |
| **Global Options**                         |                              |                                 |                                                |                                                 |                                                   |
| Path to the Git repository                 | `directory`                  | `-d, --directory <PATH>`        | `directory`                                    | `DOXXER_DIRECTORY`                              | `.` (current directory)                           |
| Path to config file or directory           | `config_path`                | `-c, --config <PATH>`           | N/A (CLI only)                                 | N/A (CLI only)                                  | (none)                                            |
| Regex to filter relevant Git tags          | `filter.tag`                 | `-t, --tag-filter <REGEX>`      | `filter.tag`                                   | `DOXXER_FILTER__TAG`                            | `""` (empty string, no filter)                    |
| Output format for the version              | `output.format`              | `-f, --format <FORMAT>`         | `output.format`                                | `DOXXER_OUTPUT__FORMAT`                         | `plain`                                           |
| Template for the resulting version string  | `output.template`            | `-o, --template <TEMPLATE>`     | `output.template`                              | `DOXXER_OUTPUT__TEMPLATE`                       | `{version}`                                       |
| **Command: `current` & `next`**            |                              |                                 |                                                |                                                 |                                                   |
| Field/part of the version to output        | `field`                      | `-F, --field <FIELD>`           | N/A (CLI only)                                 | N/A (CLI only)                                  | (outputs full version)                            |
| **Strategy: `major`, `minor`, `patch`**    |                              |                                 | `[next.major]`, `[next.minor]`, `[next.patch]` | `DOXXER_NEXT__MAJOR__...`, etc.                 |                                                   |
| Version bump increment                     | `increment`                  | `--increment <NUM>`             | `increment`                                    | `...INCREMENT`                                  | `1`                                               |
| Build metadata template                    | `build_metadata.template`    | `--build-metadata-template <TPL>` | `build_metadata.template`                    | `...BUILD_METADATA__TEMPLATE`                 | `""` (empty string)                               |
| **Strategy: `prerelease`**                 |                              |                                 | `[next.prerelease]`                            | `DOXXER_NEXT__PRERELEASE__...`                  |                                                   |
| Prerelease identifier                      | `identifier`                 | `<IDENTIFIER>` (positional)     | `prerelease.identifier`                        | `...IDENTIFIER`                                 | `build`                                           |
| Template for prerelease part               | `prerelease_template`        | `--prerelease-template <TPL>`   | `prerelease.template`                        | `...PRERELEASE_TEMPLATE`                        | `{identifier}.{inc}`                              |
| Build metadata template                    | `build_metadata.template`    | `--build-metadata-template <TPL>` | `build_metadata.template`                    | `...BUILD_METADATA__TEMPLATE`                 | `""` (empty string)                               |
| **Strategy: `premajor`, `preminor`, `prepatch`** |                        |                                 | `[next.premajor]`, etc.                        | `DOXXER_NEXT__PREMAJOR__...`, etc.              |                                                   |
| Version bump increment                     | `increment`                  | `--increment <NUM>`             | `increment`                                    | `...INCREMENT`                                  | `1`                                               |
| Prerelease identifier                      | `identifier`                 | `<IDENTIFIER>` (positional)     | `prerelease.identifier`                        | `...IDENTIFIER`                                 | `build`                                           |
| Template for prerelease part               | `prerelease_template`        | `--prerelease-template <TPL>`   | `prerelease.template`                        | `...PRERELEASE_TEMPLATE`                        | `{identifier}.{inc}`                              |
| Build metadata template                    | `build_metadata.template`    | `--build-metadata-template <TPL>` | `build_metadata.template`                    | `...BUILD_METADATA__TEMPLATE`                 | `""` (empty string)                               |
| **Strategy: `dev`**                        |                              |                                 | `[next.dev]`                                   | `DOXXER_NEXT__DEV__...`                         |                                                   |
| Prerelease identifier                      | `identifier`                 | `<IDENTIFIER>` (positional)     | `prerelease.identifier`                        | `...IDENTIFIER`                                 | `dev`                                             |
| Template for prerelease part               | `prerelease_template`        | `--prerelease-template <TPL>`   | `prerelease.template`                        | `...PRERELEASE_TEMPLATE`                        | `{pre}.{identifier}.{distance}`                   |
| Build metadata template                    | `build_metadata.template`    | `--build-metadata-template <TPL>` | `build_metadata.template`                    | `...BUILD_METADATA__TEMPLATE`                 | `{hash}`                                          |

**Notes on Config File Keys & Environment Variables for Strategies:**
*   For strategies like `major`, `minor`, `patch`, `premajor`, `preminor`, `prepatch`, `prerelease`, and `dev`, the `Config File Key` shown is relative to the strategy's table in `doxxer.toml`. For example, for the `major` strategy, `increment` is specified as `increment = 1` under the `[next.major]` table.
*   The `Environment Variable` column for strategies uses `...` as a placeholder for the capitalized strategy name part. For example, for `increment` under the `major` strategy, the variable would be `DOXXER_NEXT__MAJOR__INCREMENT`. For `identifier` under the `dev` strategy, it would be `DOXXER_NEXT__DEV__IDENTIFIER`.
*   CLI flags like `--increment`, `--prerelease-template`, and `--build-metadata-template` are available for the specific `next` strategies that support them. Clap auto-generates short versions of flags (e.g., `-i` for `--increment`) if they don't conflict; refer to `doxxer next <STRATEGY> --help` for the exact short flags.

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
    export DOXXER_OUTPUT__FORMAT=json
    export DOXXER_NEXT__MAJOR__INCREMENT=2
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
    *(Note: Default output template is `{version}`. If `DOXXER_OUTPUT__TEMPLATE` was also set, it would be used.)*

*   **Set a specific prerelease identifier and template for the `dev` strategy:**
    ```bash
    export DOXXER_NEXT__DEV__PRERELEASE_IDENTIFIER="snapshot"
    export DOXXER_NEXT__DEV__PRERELEASE_TEMPLATE="{identifier}.{distance}"
    # Assuming latest tag v1.2.3, 5 commits since tag
    doxxer next dev
    ```
    This would result in `1.2.3-snapshot.5+{hash}` (default build metadata for dev is `{hash}`).
    To also customize build metadata: `export DOXXER_NEXT__DEV__BUILD_METADATA_TEMPLATE="build.{hash}"`

### 4. Common Use Cases & Specific Scenarios

*   **Always output JSON:**
    *   In `doxxer.toml`:
        ```toml
        [output]
        format = "json"
        ```
    *   Or with env var: `export DOXXER_OUTPUT__FORMAT=json`
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
    A: Global options (like `directory`, `output.format`, `filter.tag`) can be set at the top level of your `doxxer.toml` file or within general tables like `[output]` and `[filter]`. These apply unless overridden by more specific configurations (command-specific, environment variables, or CLI arguments). For environment variables, use the base prefixes (e.g., `DOXXER_OUTPUT__FORMAT`, `DOXXER_FILTER__TAG`). Refer to the "Configuration Priority" and "Configuration Files" sections for more details.

5.  **Q: What's the easiest way to always output JSON (or another format)?**
    A: The most persistent way is to set it in your `doxxer.toml` file:
    ```toml
    [output]
    format = "json"
    ```
    Alternatively, you can export an environment variable: `export DOXXER_OUTPUT__FORMAT=json`. If you only need it for a single command execution, use the `-f json` (or `--format json`) CLI flag.

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