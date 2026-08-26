## 0.2.0

#### Breaking

- Requires proto >= 0.56.0. Older versions do not send the `working_dir` context field the plugin now expects.

#### Fixes

- Fixed the `missing field tool_dir` error on recent proto releases, which removed the deprecated `tool_dir` field from the unresolved plugin context.

#### Improvements

- Updated to `proto_pdk` 0.35: `ToolContext` -> `PluginContext`, and the tool config schema moved from `RegisterToolOutput.config_schema` to the `define_tool_config` function.
- Replaced semver comparisons with proto's own `Version` type.
- Pinned `schematic` to 0.20 to keep a single `schematic_types` version in the dependency tree.

## 0.1.1

#### Fixes

- Fixed typo in error message ("Plase" → "Please")

#### Improvements

- Replaced `unreachable!()` with proper error returns in `download_prebuilt`
- Replaced `Path`-based URL parsing with string operations in `add_versions_for_channel`
- Flattened nested `if let` chain with `let-else` and early continues
- Added comments for version threshold constants
- Enhanced README with badges, usage examples, version detection, supported platforms table, and proto version requirement
- Added GitHub Actions release workflow
- Removed redundant `build-wasm.sh` script

## 0.1.0

- Initial release
