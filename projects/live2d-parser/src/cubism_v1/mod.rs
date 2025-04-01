use serde::{Deserialize, Serialize};

pub mod moc;

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
pub struct ModelJson {
    pub version: String,
    pub model: String,
    pub textures: Vec<String>,
    // pub bubble: Option<_>,
    pub motions: Live2dMotions,
    pub expressions: Vec<Live2DExpression>,
    pub physics: String,
    // pub intimacy_param: Option<_>,
}
