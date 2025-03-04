use proto_pdk_test_utils::*;

mod dart_tool {
    use super::*;

    generate_download_install_tests!("dart-test", "3.7.1");

    #[tokio::test(flavor = "multi_thread")]
    async fn supports_macos_arm64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin_arm = sandbox
            .create_plugin_with_config("dart-test", |config| {
                config.host(HostOS::MacOS, HostArch::Arm64);
            })
            .await;

        assert_eq!(
            plugin_arm
                .download_prebuilt(DownloadPrebuiltInput {
                    context: ToolContext {
                        version: VersionSpec::parse("3.7.1").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await,
            DownloadPrebuiltOutput {
                download_url:
                    "https://storage.googleapis.com/dart-archive/channels/stable/release/3.7.1/sdk/dartsdk-macos-arm64-release.zip"
                        .into(),
                checksum_url: Some("https://storage.googleapis.com/dart-archive/channels/stable/release/3.7.1/sdk/dartsdk-macos-arm64-release.zip.sha256sum".into()),
                ..Default::default()
            }
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn locates_unix_bin() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("dart-test", |config| {
                config.host(HostOS::Linux, HostArch::Arm64);
            })
            .await;

        assert_eq!(
            plugin
                .locate_executables(LocateExecutablesInput {
                    context: ToolContext {
                        version: VersionSpec::parse("3.7.1").unwrap(),
                        ..Default::default()
                    },
                })
                .await
                .exes
                .get("dart")
                .unwrap()
                .exe_path,
            Some("dart-sdk/bin/dart".into())
        );
        assert_eq!(
            plugin
                .locate_executables(LocateExecutablesInput {
                    context: ToolContext {
                        version: VersionSpec::parse("3.7.1").unwrap(),
                        ..Default::default()
                    },
                })
                .await
                .exes
                .get("dartaotruntime")
                .unwrap()
                .exe_path,
            Some("dart-sdk/bin/dartaotruntime".into())
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn locates_windows_bin() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("dart-test", |config| {
                config.host(HostOS::Windows, HostArch::X64);
            })
            .await;

        assert_eq!(
            plugin
                .locate_executables(LocateExecutablesInput {
                    context: ToolContext {
                        version: VersionSpec::parse("3.7.1").unwrap(),
                        ..Default::default()
                    },
                })
                .await
                .exes
                .get("dart")
                .unwrap()
                .exe_path,
            Some("dart-sdk/bin/dart.exe".into())
        );
        assert_eq!(
            plugin
                .locate_executables(LocateExecutablesInput {
                    context: ToolContext {
                        version: VersionSpec::parse("3.7.1").unwrap(),
                        ..Default::default()
                    },
                })
                .await
                .exes
                .get("dartaotruntime")
                .unwrap()
                .exe_path,
            Some("dart-sdk/bin/dartaotruntime.exe".into())
        );
    }
}
