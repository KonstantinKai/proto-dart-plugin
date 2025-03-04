#[derive(Debug, schematic::Schematic, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct DartPluginConfig {
    pub dist_url: String,
}

impl Default for DartPluginConfig {
    fn default() -> Self {
        Self {
            dist_url: "https://storage.googleapis.com/dart-archive/channels/{channel}/release/{version}/sdk/dartsdk-{platform}-{arch}-release.zip".into(),
        }
    }
}
