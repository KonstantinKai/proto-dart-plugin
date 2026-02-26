use std::collections::HashMap;

use extism_pdk::*;
use proto_pdk::*;
use schematic::SchemaBuilder;

use crate::{DartLatest, DartPluginConfig, DartPrefixes, PubspecYaml};

#[host_fn]
extern "ExtismHost" {
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandInput>;
}

static NAME: &str = "Dart";

#[plugin_fn]
pub fn register_tool(Json(_): Json<RegisterToolInput>) -> FnResult<Json<RegisterToolOutput>> {
    Ok(Json(RegisterToolOutput {
        name: NAME.into(),
        minimum_proto_version: Some(Version::new(0, 46, 0)),
        type_of: PluginType::Language,
        default_install_strategy: InstallStrategy::DownloadPrebuilt,
        config_schema: Some(SchemaBuilder::build_root::<DartPluginConfig>()),
        plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(),
        ..RegisterToolOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let env = get_host_environment()?;
    let mut output = LoadVersionsOutput::default();

    add_versions_for_channel("stable", &mut output, &env)?;
    add_versions_for_channel("beta", &mut output, &env)?;

    Ok(Json(output))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_host_environment()?;

    let version_spec = input.context.version;

    check_version_for_os_and_arch(&env, &version_spec)?;

    if version_spec.is_canary() {
        return Err(plugin_err!(PluginError::Message(format!(
            "{NAME} does not support canary/nightly versions. Please use `proto install dart beta` instead"
        ))));
    }

    let config = get_tool_config::<DartPluginConfig>()?;

    let platform = match env.os {
        HostOS::Linux => "linux",
        HostOS::MacOS => "macos",
        HostOS::Windows => "windows",
        _ => {
            return Err(PluginError::UnsupportedOS {
                tool: NAME.to_owned(),
                os: env.os.to_string(),
            }
            .into())
        }
    };
    let arch = match env.arch {
        HostArch::Riscv64 => "riscv64",
        HostArch::X86 => "ia32",
        HostArch::X64 => "x64",
        HostArch::Arm => "arm",
        HostArch::Arm64 => "arm64",
        _ => {
            return Err(plugin_err!(PluginError::UnsupportedTarget {
                tool: NAME.to_owned(),
                arch: env.arch.to_string(),
                os: env.os.to_string(),
            }))
        }
    };
    let version = version_spec.as_version().unwrap();
    let channel = if version.pre.is_empty() {
        "stable"
    } else {
        "beta"
    };

    let download_url = config
        .dist_url
        .replace("{channel}", channel)
        .replace("{version}", version.to_string().as_str())
        .replace("{platform}", platform)
        .replace("{arch}", arch);
    let checksum_url = format!("{}.sha256sum", download_url);

    Ok(Json(DownloadPrebuiltOutput {
        download_url,
        checksum_url: Some(checksum_url),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([
            (
                "dart".into(),
                ExecutableConfig::new_primary(
                    env.os
                        .for_native("dart-sdk/bin/dart", "dart-sdk/bin/dart.exe"),
                ),
            ),
            (
                "dartaotruntime".into(),
                ExecutableConfig::new(env.os.for_native(
                    "dart-sdk/bin/dartaotruntime",
                    "dart-sdk/bin/dartaotruntime.exe",
                )),
            ),
        ]),
        globals_lookup_dirs: vec!["$PUB_CACHE/bin".into(), "$HOME/.pub-cache/bin".into()],
        ..LocateExecutablesOutput::default()
    }))
}

#[plugin_fn]
pub fn detect_version_files(_: ()) -> FnResult<Json<DetectVersionOutput>> {
    Ok(Json(DetectVersionOutput {
        files: vec!["pubspec.yml".into(), "pubspec.yaml".into()],
        ignore: vec![],
    }))
}

#[plugin_fn]
pub fn parse_version_file(
    Json(input): Json<ParseVersionFileInput>,
) -> FnResult<Json<ParseVersionFileOutput>> {
    let mut version = None;

    if input.file.starts_with("pubspec") {
        let pubspec: PubspecYaml = serde_yml::from_str(&input.content)?;

        if let Some(env) = pubspec.environment {
            if let Some(constraint) = env.sdk {
                version = Some(UnresolvedVersionSpec::parse(constraint)?);
            }
        }
    }

    Ok(Json(ParseVersionFileOutput { version }))
}

pub fn check_version_for_os_and_arch(
    env: &HostEnvironment,
    version_spec: &VersionSpec,
) -> FnResult<()> {
    let version = version_spec.as_version().unwrap();

    let unresolved_version_spec_option = match env.os {
        // Linux ia32 (x86) builds were dropped starting from Dart 3.8.0
        // Linux ARM builds first appeared in 1.12.0
        // Linux ARM64 builds first appeared in 1.23.0
        // Linux RISC-V 64 beta builds first appeared in 3.0.0-290.2.beta, stable in 3.3.0
        HostOS::Linux => match env.arch {
            HostArch::X86
                if version_spec
                    .ge(VersionSpec::Semantic(SemVer(Version::new(3, 8, 0))).as_ref()) =>
            {
                UnresolvedVersionSpec::parse("<3.8.0").ok()
            }
            HostArch::Arm
                if version_spec
                    .lt(VersionSpec::Semantic(SemVer(Version::new(1, 12, 0))).as_ref()) =>
            {
                UnresolvedVersionSpec::parse(">=1.12.0").ok()
            }
            HostArch::Arm64
                if version_spec
                    .lt(VersionSpec::Semantic(SemVer(Version::new(1, 23, 0))).as_ref()) =>
            {
                UnresolvedVersionSpec::parse(">=1.23.0").ok()
            }
            HostArch::Riscv64
                if !version.pre.is_empty()
                    && version_spec.lt(VersionSpec::Semantic(SemVer(
                        Version::parse("3.0.0-290.2.beta").ok().unwrap(),
                    ))
                    .as_ref()) =>
            {
                UnresolvedVersionSpec::parse(">=3.0.0-290.2.beta").ok()
            }
            HostArch::Riscv64
                if version.pre.is_empty()
                    && version_spec
                        .lt(VersionSpec::Semantic(SemVer(Version::new(3, 3, 0))).as_ref()) =>
            {
                UnresolvedVersionSpec::parse(">=3.3.0").ok()
            }
            _ => None::<UnresolvedVersionSpec>,
        },
        // macOS ia32 (x86) builds were dropped starting from Dart 2.8.0
        // macOS ARM64 (Apple Silicon) builds first appeared in 2.14.1
        HostOS::MacOS => match env.arch {
            HostArch::X86
                if version_spec
                    .gt(VersionSpec::Semantic(SemVer(Version::new(2, 7, 0))).as_ref()) =>
            {
                UnresolvedVersionSpec::parse("<2.8.0").ok()
            }
            HostArch::Arm64
                if version_spec
                    .lt(VersionSpec::Semantic(SemVer(Version::new(2, 14, 1))).as_ref()) =>
            {
                UnresolvedVersionSpec::parse(">=2.14.1").ok()
            }
            _ => None::<UnresolvedVersionSpec>,
        },
        // Windows ia32 (x86) builds were dropped starting from Dart 2.8.0
        // Windows ARM64 beta builds first appeared in 3.2.0-42.2.beta, stable in 3.3.0
        HostOS::Windows => match env.arch {
            HostArch::X86
                if version_spec
                    .gt(VersionSpec::Semantic(SemVer(Version::new(2, 7, 0))).as_ref()) =>
            {
                UnresolvedVersionSpec::parse("<2.8.0").ok()
            }
            HostArch::Arm64
                if !version.pre.is_empty()
                    && version_spec.lt(VersionSpec::Semantic(SemVer(
                        Version::parse("3.2.0-42.2.beta").ok().unwrap(),
                    ))
                    .as_ref()) =>
            {
                UnresolvedVersionSpec::parse(">=3.2.0-42.2.beta").ok()
            }
            HostArch::Arm64
                if version.pre.is_empty()
                    && version_spec
                        .lt(VersionSpec::Semantic(SemVer(Version::new(3, 3, 0))).as_ref()) =>
            {
                UnresolvedVersionSpec::parse(">=3.3.0").ok()
            }
            _ => None::<UnresolvedVersionSpec>,
        },
        _ => UnresolvedVersionSpec::parse("0.0.0").ok(),
    };

    match unresolved_version_spec_option {
        Some(unresolved_version_spec) => match unresolved_version_spec {
            UnresolvedVersionSpec::Req(req) => {
                let arch = env.arch.to_string();

                Err(plugin_err!(PluginError::Message(format!(
                    "Unable to install {NAME}@{version} for the current architecture {arch} and os. Require {req}"
                ))))
            }
            _ => Err(PluginError::UnsupportedOS {
                tool: NAME.to_owned(),
                os: env.os.to_string(),
            }
            .into()),
        },
        _ => Ok(()),
    }
}

pub fn add_versions_for_channel(
    channel: &str,
    output: &mut LoadVersionsOutput,
    env: &HostEnvironment,
) -> FnResult<()> {
    let latest = fetch_json::<String, DartLatest>(format!(
        "https://storage.googleapis.com/dart-archive/channels/{channel}/release/latest/VERSION"
    ))?;
    let res = fetch_json::<String, DartPrefixes>(format!("https://storage.googleapis.com/storage/v1/b/dart-archive/o?delimiter=%2F&prefix=channels%2F{channel}%2Frelease%2F&alt=json"))?;

    // Prefixes are GCS paths like "channels/stable/release/3.7.1/"
    for item in res.prefixes.iter() {
        let version_as_str = item.trim_end_matches('/').rsplit('/').next().unwrap_or("");

        let Ok(version_spec) = VersionSpec::parse(version_as_str) else {
            continue;
        };

        if version_spec.as_version().is_none()
            || output.versions.contains(&version_spec)
            || check_version_for_os_and_arch(env, &version_spec).is_err()
        {
            continue;
        }

        output.versions.push(version_spec.clone());

        if latest.version == version_as_str {
            output
                .aliases
                .insert(channel.to_string(), version_spec.to_unresolved_spec());

            if channel == "stable" {
                output
                    .aliases
                    .insert("latest".into(), version_spec.to_unresolved_spec());
                output.latest = Some(version_spec.to_unresolved_spec());
            }
        }
    }

    Ok(())
}
