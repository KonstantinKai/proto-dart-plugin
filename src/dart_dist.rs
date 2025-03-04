use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DartLatest {
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct DartPrefixes {
    pub prefixes: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct PubspecYamlEnvField {
    pub sdk: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PubspecYaml {
    pub name: String,
    pub environment: Option<PubspecYamlEnvField>,
}
