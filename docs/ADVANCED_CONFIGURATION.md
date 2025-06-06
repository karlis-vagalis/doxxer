# Advanced Configuration Details

This document provides advanced configuration details for doxxer, including the comprehensive options table and detailed examples.

## Configuration Options Table

The following table summarizes all available configuration options, their corresponding CLI flags, configuration file keys (within `doxxer.toml` or `.doxxer.toml`), environment variables, and default values.

| Purpose                                    | Option Name (Conceptual)     | CLI Flag(s) / Arg              | Config File Key (`[table].key`)                | Environment Variable (`DOXXER__...`)             | Default Value                                     |
| ------------------------------------------ | ---------------------------- | ------------------------------- | ---------------------------------------------- | ----------------------------------------------- | ------------------------------------------------- |
| **Global Options**                         |                              |                                 |                                                |                                                 |                                                   |
| Path to the Git repository                 | `directory`                  | `-d, --directory <PATH>`        | `directory`                                    | `DOXXER__DIRECTORY`                              | `.` (current directory)                           |
| Path to config file or directory           | `config_path`                | `-c, --config <PATH>`           | N/A (CLI only)                                 | N/A (CLI only)                                  | (none)                                            |
| Regex to filter relevant Git tags          | `filter.tag`                 | `-t, --tag-filter <REGEX>`      | `filter.tag`                                   | `DOXXER__FILTER__TAG`                            | `""` (empty string, no filter)                    |
| Output format for the version              | `output.format`              | `-f, --format <FORMAT>`         | `output.format`                                | `DOXXER__OUTPUT__FORMAT`                         | `plain`                                           |
| Template for the resulting version string  | `output.template`            | `-o, --template <TEMPLATE>`     | `output.template`                              | `DOXXER__OUTPUT__TEMPLATE`                       | `{version}`                                       |
| **Command: `current` & `next`**            |                              |                                 |                                                |                                                 |                                                   |
| Field/part of the version to output        | `field`                      | `-F, --field <FIELD>`           | N/A (CLI only)                                 | N/A (CLI only)                                  | (outputs full version)                            |
| **Strategy: `major`, `minor`, `patch`**    |                              |                                 | `[next.major]`, `[next.minor]`, `[next.patch]` | `DOXXER__NEXT__MAJOR__...`, etc.                 |                                                   |
| Version bump increment                     | `increment`                  | `--increment <NUM>`             | `increment`                                    | `...INCREMENT`                                  | `1`                                               |
| Build metadata template                    | `build_metadata.template`    | `--build-metadata-template <TPL>` | `build_metadata.template`                    | `...BUILD_METADATA__TEMPLATE`                 | `""` (empty string)                               |
| **Strategy: `prerelease`**                 |                              |                                 | `[next.prerelease]`                            | `DOXXER__NEXT__PRERELEASE__...`                  |                                                   |
| Prerelease identifier                      | `identifier`                 | `<IDENTIFIER>` (positional)     | `prerelease.identifier`                        | `...IDENTIFIER`                                 | `build`                                           |
| Template for prerelease part               | `prerelease_template`        | `--prerelease-template <TPL>`   | `prerelease.template`                        | `...PRERELEASE_TEMPLATE`                        | `{identifier}.{inc}`                              |
| Build metadata template                    | `build_metadata.template`    | `--build-metadata-template <TPL>` | `build_metadata.template`                    | `...BUILD_METADATA__TEMPLATE`                 | `""` (empty string)                               |
| **Strategy: `premajor`, `preminor`, `prepatch`** |                        |                                 | `[next.premajor]`, etc.                        | `DOXXER__NEXT__PREMAJOR__...`, etc.              |                                                   |
| Version bump increment                     | `increment`                  | `--increment <NUM>`             | `increment`                                    | `...INCREMENT`                                  | `1`                                               |
| Prerelease identifier                      | `identifier`                 | `<IDENTIFIER>` (positional)     | `prerelease.identifier`                        | `...IDENTIFIER`                                 | `build`                                           |
| Template for prerelease part               | `prerelease_template`        | `--prerelease-template <TPL>`   | `prerelease.template`                        | `...PRERELEASE_TEMPLATE`                        | `{identifier}.{inc}`                              |
| Build metadata template                    | `build_metadata.template`    | `--build-metadata-template <TPL>` | `build_metadata.template`                    | `...BUILD_METADATA__TEMPLATE`                 | `""` (empty string)                               |
| **Strategy: `dev`**                        |                              |                                 | `[next.dev]`                                   | `DOXXER__NEXT__DEV__...`                         |                                                   |
| Prerelease identifier                      | `identifier`                 | `<IDENTIFIER>` (positional)     | `prerelease.identifier`                        | `...IDENTIFIER`                                 | `dev`                                             |
| Template for prerelease part               | `prerelease_template`        | `--prerelease-template <TPL>`   | `prerelease.template`                        | `...PRERELEASE_TEMPLATE`                        | `{pre}.{identifier}.{distance}`                   |
| Build metadata template                    | `build_metadata.template`    | `--build-metadata-template <TPL>` | `build_metadata.template`                    | `...BUILD_METADATA__TEMPLATE`                 | `{hash}`                                          |

**Notes on Config File Keys & Environment Variables for Strategies:**
*   For strategies like `major`, `minor`, `patch`, `premajor`, `preminor`, `prepatch`, `prerelease`, and `dev`, the `Config File Key` shown is relative to the strategy's table in `doxxer.toml`. For example, for the `major` strategy, `increment` is specified as `increment = 1` under the `[next.major]` table.
*   The `Environment Variable` column for strategies uses `...` as a placeholder for the capitalized strategy name part. For example, for `increment` under the `major` strategy, the variable would be `DOXXER__NEXT__MAJOR__INCREMENT`. For `identifier` under the `dev` strategy, it would be `DOXXER__NEXT__DEV__IDENTIFIER`.
*   CLI flags like `--increment`, `--prerelease-template`, and `--build-metadata-template` are available for the specific `next` strategies that support them. Clap auto-generates short versions of flags (e.g., `-i` for `--increment`) if they don't conflict; refer to `doxxer next <STRATEGY> --help` for the exact short flags.

## Template Variables Details

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

## Detailed Usage Examples

(Note: `<hash>` in outputs refers to a short commit hash like `a1b2c3d`.)

### 1. Using a `doxxer.toml` Configuration File (Full Example)

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

### 2. Overriding Settings with CLI Arguments (Extended)

CLI arguments take precedence over `doxxer.toml` settings.

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

### 4. Common Use Cases & Specific Scenarios (Additional)

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

### Using Doxxer in Docker

#### Using Doxxer as a Base Image
If you want to include the `doxxer` binary inside your custom Docker image, you can copy it from the official image:
```dockerfile
FROM ghcr.io/karlis-vagalis/doxxer:latest AS base
# ... your Dockerfile lines ...
COPY --from=base /bin/doxxer /usr/local/bin/doxxer
# ... rest of your Dockerfile ...
```
This copies the `doxxer` binary to `/usr/local/bin/doxxer` in your new image.

## Frequently Asked Questions (FAQ)

1.  **Q: Why does the default `next dev` strategy sometimes append a new prerelease identifier if one already exists (e.g., `1.0.0-alpha.1` becomes `1.0.0-alpha.1.dev.5`)?**
    A: This behavior is intentional for the `dev` strategy. Its default prerelease template is `"{pre}.{identifier}.{distance}"`. The `{pre}` variable captures the *entire existing* prerelease string. This design allows you to see the lineage from a previous prerelease tag (like `alpha.1`) while also appending the `dev` specific information (identifier and commit distance). It's aimed at providing maximum context during development. If you prefer a different format for `dev` versions, you can customize `next.dev.prerelease_template` and `next.dev.identifier` in your configuration.

2.  **Q: How do I set an option globally for all commands?**
    A: Global options (like `directory`, `output.format`, `filter.tag`) can be set at the top level of your `doxxer.toml` file or within general tables like `[output]` and `[filter]`. These apply unless overridden by more specific configurations (command-specific, environment variables, or CLI arguments). For environment variables, use the base prefixes (e.g., `DOXXER__OUTPUT__FORMAT`, `DOXXER__FILTER__TAG`). Refer to the "Configuration Priority" and "Configuration Files" sections for more details.

3.  **Q: What's the easiest way to always output JSON (or another format)?**
    A: The most persistent way is to set it in your `doxxer.toml` file:
    ```toml
    [output]
    format = "json"
    ```
    Alternatively, you can export an environment variable: `export DOXXER__OUTPUT__FORMAT=json`. If you only need it for a single command execution, use the `-f json` (or `--format json`) CLI flag.

4.  **Q: Can I have different prerelease identifiers for `patch` vs. `minor` bumps when using strategies like `prepatch` or `preminor`?**
    A: Yes! You can define this in your `doxxer.toml` by targeting the specific strategy:
    ```toml
    [next.prepatch]
    identifier = "rc" # e.g., for release candidates on patches, version like 1.2.3-rc.1

    [next.preminor]
    identifier = "beta" # e.g., for beta releases on minor bumps, version like 1.3.0-beta.1
    ```
    The same principle applies to other strategy-specific settings like `increment` values or custom `prerelease_template` and `build_metadata_template`. Refer to the "Configuration Options Table" for all available configuration keys.

5.  **Q: Why does my `tag_filter` not seem to work?**
    A: Common reasons include:
    *   **Regex Syntax:** Ensure your regular expression is valid (e.g., correctly escaped in TOML) and accurately describes the tags you want `doxxer` to consider. Test your regex with a dedicated tool if unsure.
    *   **Matching Scope:** The regex usually needs to match the entire part of the tag name you intend to parse for versioning. For example, to match tags like `v1.0.0`, `v1.2.3`, a regex like `^v(\d+\.\d+\.\d+)$` is appropriate if you want to capture the SemVer part.
    *   **Configuration Priority:** Double-check that your intended `tag_filter` isn't being overridden by a higher-priority source (like a CLI argument or an environment variable). See "Configuration Priority".
    *   **No Matching Tags:** There might genuinely be no tags in your repository that match the filter and also contain a parsable SemVer string.
    *   **Interaction with SemVer Parsing:** `doxxer` first filters tags using `tag_filter`, then attempts to parse a SemVer string from the *matching tag name*. If your filter matches a tag that isn't itself a valid SemVer string (or doesn't start with one, possibly after stripping a 'v'), it will be ignored.

6.  **Q: How does `tag_filter` interact with SemVer parsing?**
    A: `doxxer` first applies the `tag_filter` regex to all Git tag names. For each tag name that matches the filter, `doxxer` then attempts to find and parse a Semantic Version from that tag name. It can handle common prefixes like 'v' automatically (e.g., `v1.2.3` is parsed as `1.2.3`). If your `tag_filter` matches a tag name from which a valid SemVer string cannot be extracted, that tag will be ignored when determining the latest version. For example, if `tag_filter = "release-(.*)"` matches `release-my-app-1.2.3`, doxxer will attempt to parse `my-app-1.2.3` as SemVer, which would likely fail unless the filter was more specific like `release-my-app-(v?\d+\.\d+\.\d+)`. It's best if your filter is designed to match tags that are clearly SemVer compatible.

7.  **Q: Why do strategies like `premajor` or `prerelease` default to `build.1`-style prereleases, and `dev` to a different style?**
    A: `doxxer` has distinct default behaviors for "release-oriented" prereleases versus "development" prereleases:
    *   For strategies like `prerelease`, `premajor`, `preminor`, and `prepatch`, the default prerelease identifier is `"build"` and the default template is `"{identifier}.{inc}"` (e.g., `build.1`, `build.2`). This provides a generic prerelease sequence.
    *   For the `dev` strategy (which is also the default for `doxxer next` if no strategy is given), the default identifier is `"dev"` and the template is `"{pre}.{identifier}.{distance}"` (e.g., `dev.5` or `alpha.1.dev.5`). This template is designed to append to existing prereleases and include commit distance, offering more context during active development.
    These defaults can all be overridden in the configuration if you prefer a different style for any strategy (see the "Configuration Options Table"). For instance, a simple `doxxer next patch` (without `pre`) will not add any prerelease by default.

8.  **Q: How are multiple prerelease identifiers handled if the current version already has one and I use `next dev`?**
    A: The `dev` strategy's default prerelease template is `"{pre}.{identifier}.{distance}"`. The `{pre}` variable is populated with the *entire existing prerelease string* of the current version.
    For example, if your current tagged version is `1.2.3-alpha.1` and you run `doxxer next dev` (assuming the new identifier is `dev` and commit `distance` is 5):
    - `{pre}` will be `alpha.1`.
    - `{identifier}` will be `dev`.
    - `{distance}` will be `5`.
    The resulting prerelease string will be `alpha.1.dev.5`, making the full version `1.2.3-alpha.1.dev.5`.
    Other strategies (like `major`, `minor`, `patch`, `prerelease`, `premajor`, etc.) typically replace the existing prerelease segment entirely or increment parts of it, rather than appending new identifiers after the full existing segment in this way. You can, of course, customize the `prerelease_template` for any strategy to achieve different concatenation effects if needed.
