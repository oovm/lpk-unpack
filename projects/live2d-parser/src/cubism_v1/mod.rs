use serde::{Deserialize, Serialize};

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
