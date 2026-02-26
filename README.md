# Dart plugin

[![Release](https://github.com/KonstantinKai/proto-dart-plugin/actions/workflows/release.yml/badge.svg)](https://github.com/KonstantinKai/proto-dart-plugin/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A community [WASM plugin](https://moonrepo.dev/docs/proto/wasm-plugin) for [proto](https://github.com/moonrepo/proto) that manages [Dart](https://dart.dev/) SDK versions.

Requires [proto](https://github.com/moonrepo/proto) >= 0.46.0

**NOTE:** If you are using the [proto-flutter-plugin](https://github.com/KonstantinKai/proto-flutter-plugin), you don't need this plugin in most cases — Flutter bundles Dart.

## Installation

```sh
proto plugin add dart "github://KonstantinKai/proto-dart-plugin"
proto install dart
```

Or add manually to `.prototools`:

```toml
[plugins.tools]
dart = "github://KonstantinKai/proto-dart-plugin"
```

## Usage

```sh
# Install Dart
proto install dart 3.7

# Use Dart
proto run dart -- --version

# List available versions
proto versions dart

# Pin a version in the current directory
proto pin dart 3.7
```

## Version Detection

The plugin automatically detects Dart versions from:

- `pubspec.yaml` / `pubspec.yml` — reads `environment.sdk` field (supports version constraints)

## Configuration

Configure in `.prototools` under `[tools.dart]`:

```toml
[tools.dart]
# Custom download URL template (default: official Dart archive)
# Placeholders: {channel}, {version}, {platform}, {arch}
dist-url = "https://storage.googleapis.com/dart-archive/channels/{channel}/release/{version}/sdk/dartsdk-{platform}-{arch}-release.zip"
```

## Supported Platforms

| Platform | Architecture | Notes |
|----------|-------------|-------|
| Linux | x64 | All versions |
| Linux | x86 (ia32) | < 3.8.0 only |
| Linux | arm | >= 1.12.0 |
| Linux | arm64 | >= 1.23.0 |
| Linux | riscv64 | Stable >= 3.3.0, beta >= 3.0.0-290.2.beta |
| macOS | x64 | All versions |
| macOS | x86 (ia32) | < 2.8.0 only |
| macOS | arm64 | >= 2.14.1 |
| Windows | x64 | All versions |
| Windows | x86 (ia32) | < 2.8.0 only |
| Windows | arm64 | Stable >= 3.3.0, beta >= 3.2.0-42.2.beta |

## Notes

- Supports version aliases: `stable`, `beta`, `latest`
- Only includes stable and beta channel versions
- Respects platform and architecture compatibility when listing versions

## Hooks

Dart plugin does not support hooks.

## Contributing

Build the plugin:

```sh
cargo build --target wasm32-wasip1
```
