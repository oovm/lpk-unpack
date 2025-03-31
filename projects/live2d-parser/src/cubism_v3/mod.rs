use serde::{Deserialize, Serialize};

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
pub struct Struct2 {
    #[serde(rename = "File")]
    pub file: String,
    #[serde(rename = "FadeInTime")]
    pub fade_in_time: f64,
    #[serde(rename = "FadeOutTime")]
    pub fade_out_time: f64,
    #[serde(rename = "Sound")]
    pub sound: String,
}

#[derive(Serialize, Deserialize)]
pub struct Struct1 {
    #[serde(rename = "File")]
    pub file: String,
    #[serde(rename = "FadeInTime")]
    pub fade_in_time: f64,
    #[serde(rename = "FadeOutTime")]
    pub fade_out_time: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Motions {
    #[serde(rename = "Idle")]
    pub idle: Vec<Struct1>,
    #[serde(rename = "TapBody")]
    pub tap_body: Vec<Struct2>,
}

#[derive(Serialize, Deserialize)]
pub struct Struct {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "File")]
    pub file: String,
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
    #[serde(rename = "Expressions")]
    pub expressions: Vec<Struct>,
    #[serde(rename = "Motions")]
    pub motions: Motions,
    #[serde(rename = "UserData")]
    pub user_data: String,
}

#[derive(Serialize, Deserialize)]
pub struct ModelConfigV3 {
    #[serde(rename = "Version")]
    pub version: i64,
    #[serde(rename = "FileReferences")]
    pub file_references: FileReferences,
    #[serde(rename = "Groups")]
    pub groups: Vec<Struct3>,
    #[serde(rename = "HitAreas")]
    pub hit_areas: Vec<HitArea>,
}
