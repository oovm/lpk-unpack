use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod moc3;

#[derive(Serialize, Deserialize)]
pub struct HitArea {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Struct3 {
    #[serde(rename = "Target")]
    pub target: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Ids")]
    pub ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Motion {
    #[serde(rename = "File")]
    pub file: String,
    #[serde(rename = "FadeInTime", default)]
    pub fade_in_time: f64,
    #[serde(rename = "FadeOutTime", default)]
    pub fade_out_time: f64,
    #[serde(rename = "Sound", default)]
    pub sound: String,
}

#[derive(Serialize, Deserialize)]
pub struct Model3Json {
    #[serde(rename = "Version")]
    pub version: i64,
    #[serde(rename = "FileReferences")]
    pub file_references: FileReferences,
    #[serde(rename = "Groups")]
    pub groups: Vec<Struct3>,
    #[serde(rename = "HitAreas")]
    pub hit_areas: Vec<HitArea>,
}
#[derive(Serialize, Deserialize)]
pub struct FileReferences {
    #[serde(rename = "Moc")]
    pub moc: String,
    #[serde(rename = "Textures")]
    pub textures: Vec<String>,
    #[serde(rename = "Physics")]
    pub physics: String,
    #[serde(rename = "Pose")]
    pub pose: String,
    #[serde(rename = "DisplayInfo")]
    pub display_info: String,
    #[serde(rename = "Expressions", default)]
    pub expressions: Vec<Expression>,
    #[serde(rename = "Motions")]
    pub motions: HashMap<String, Vec<Motion>>,
    #[serde(rename = "UserData")]
    pub user_data: String,
}

#[derive(Serialize, Deserialize)]
pub struct Expression {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "File")]
    pub file: String,
}
