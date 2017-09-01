use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct LpkConfig {
    #[serde(rename = "lpkFile")]
    pub lpk_file: String,
    pub file: String,
    #[serde(rename = "previewFile")]
    pub preview_file: String,
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[serde(rename = "type")]
    pub r#type: i64,
    #[serde(rename = "stereoMode")]
    pub stereo_mode: i64,
    pub title: String,
    pub author: String,
    pub description: String,
    #[serde(rename = "metaData")]
    pub meta_data: String,
    pub key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Struct1 {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Struct {
    pub id: String,
    pub character: String,
    pub avatar: String,
    pub costume: Vec<Struct1>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MLveConfig {
    #[serde(rename = "type")]
    pub r#type: String,
    pub name: String,
    pub id: String,
    pub encrypt: String,
    pub version: String,
    pub list: Vec<Struct>,
}
