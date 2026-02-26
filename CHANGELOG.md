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
