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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MLveConfig {
    #[serde(rename = "type")]
    pub r#type: String,
    pub name: String,
    pub id: String,
    pub encrypt: String,
    pub version: String,
    pub list: Vec<LpkCharacter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LpkCostume {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LpkCharacter {
    pub id: String,
    pub character: String,
    pub avatar: String,
    pub costume: Vec<LpkCostume>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Live2DExpression {
    pub name: String,
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Live2dTapMotion {
    pub file: String,
    // pub time_limit: Option<_>,
    // pub intimacy: Option<_>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Live2dMotions {
    pub tap: Vec<Live2dTapMotion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Live2dConfig {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub textures: Vec<String>,
    // pub bubble: Option<_>,
    #[serde(default)]
    pub motions: Live2dMotions,
    #[serde(default)]
    pub expressions: Vec<Live2DExpression>,
    #[serde(default)]
    pub physics: String,
    // pub intimacy_param: Option<_>,
}
