# Dart plugin

[Dart](https://dart.dev/) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Installation

**NOTE:** if you are using [flutter](https://github.com/KonstantinKai/proto-flutter-plugin) tool you don't need to install this tool in most cases

Add the following to `.prototools`.

```toml
[plugins]
dart = "github://KonstantinKai/proto-dart-plugin"
```

Or

```sh
proto plugin add dart github://KonstantinKai/proto-dart-plugin
```

## Configuration

Dart plugin can be configured with a `.prototools` file.

- `dist-url` (string) - The dist URL to download dart SDK archives.

```toml
[tools.dart]
dist-url = "https://storage.googleapis.com/dart-archive/channels/{channel}/release/{version}/sdk/dartsdk-{platform}-{arch}-release.zip" # default
```

## Notes

- Dart plugin supports version aliases like `stable`, `beta`, `latest`
- Dart plugin provides only versions for stable and beta channels with Non zero MAJOR part and respects arch and os compatibility.

## Hooks

Dart plugin does not support hooks.

## Contributing

Build the plugin:

```shell
cargo build --target wasm32-wasip1
```
