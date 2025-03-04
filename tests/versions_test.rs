use proto_pdk_test_utils::*;

mod dart_tool {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn loads_versions_from_dist_url() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("dart-test").await;

        let output = plugin.load_versions(LoadVersionsInput::default()).await;

        assert!(!output.versions.is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn sets_latest_stable_beta_alias() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("dart-test").await;

        let output = plugin.load_versions(LoadVersionsInput::default()).await;

        assert!(output.latest.is_some());
        assert!(output.aliases.contains_key("latest"));
        assert_eq!(output.aliases.get("latest"), output.latest.as_ref());

        assert!(output.aliases.contains_key("stable"));
        assert_eq!(output.aliases.get("stable"), output.latest.as_ref());

        assert!(output.aliases.contains_key("beta"));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn parses_pubspec() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("dart-test").await;

        assert_eq!(
            plugin
                .parse_version_file(ParseVersionFileInput {
                    content: r#"
name: "My name"
environment:
  sdk: "<=3.6.0"
                    "#
                    .into(),
                    file: "pubspec.yaml".into(),
                    ..Default::default()
                })
                .await,
            ParseVersionFileOutput {
                version: Some(UnresolvedVersionSpec::parse("<=3.6.0").unwrap()),
            }
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    #[should_panic]
    async fn check_versions_linux_x86() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("dart-test", |config| {
                config.host(HostOS::Linux, HostArch::X86);
            })
            .await;

        let _ = plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("3.8.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await;
    }

    #[tokio::test(flavor = "multi_thread")]
    #[should_panic]
    async fn check_versions_linux_arm() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("dart-test", |config| {
                config.host(HostOS::Linux, HostArch::Arm);
            })
            .await;

        let _ = plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.11.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await;
    }

    #[tokio::test(flavor = "multi_thread")]
    #[should_panic]
    async fn check_versions_linux_arm64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("dart-test", |config| {
                config.host(HostOS::Linux, HostArch::Arm64);
            })
            .await;

        let _ = plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.22.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await;
    }

    #[tokio::test(flavor = "multi_thread")]
    #[should_panic]
    async fn check_versions_linux_riscv64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("dart-test", |config| {
                config.host(HostOS::Linux, HostArch::Riscv64);
            })
            .await;

        let _ = plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("3.2.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await;
    }

    #[tokio::test(flavor = "multi_thread")]
    #[should_panic]
    async fn check_versions_linux_riscv64_beta() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("dart-test", |config| {
                config.host(HostOS::Linux, HostArch::Riscv64);
            })
            .await;

        let _ = plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("3.0.0-290.1.beta").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await;
    }

    #[tokio::test(flavor = "multi_thread")]
    #[should_panic]
    async fn check_versions_macos_x86() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("dart-test", |config| {
                config.host(HostOS::MacOS, HostArch::X86);
            })
            .await;

        let _ = plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("2.8.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await;
    }

    #[tokio::test(flavor = "multi_thread")]
    #[should_panic]
    async fn check_versions_macos_arm64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("dart-test", |config| {
                config.host(HostOS::MacOS, HostArch::Arm64);
            })
            .await;

        let _ = plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("2.14.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await;
    }

    #[tokio::test(flavor = "multi_thread")]
    #[should_panic]
    async fn check_versions_windows_x86() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("dart-test", |config| {
                config.host(HostOS::Windows, HostArch::X86);
            })
            .await;

        let _ = plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("2.8.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await;
    }

    #[tokio::test(flavor = "multi_thread")]
    #[should_panic]
    async fn check_versions_windows_arm64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("dart-test", |config| {
                config.host(HostOS::Windows, HostArch::Arm64);
            })
            .await;

        let _ = plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("3.2.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await;
    }

    #[tokio::test(flavor = "multi_thread")]
    #[should_panic]
    async fn check_versions_windows_arm64_beta() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("dart-test", |config| {
                config.host(HostOS::Windows, HostArch::Arm64);
            })
            .await;

        let _ = plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("3.2.0-42.1.beta").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await;
    }
}
