pub mod cubism_v1;
pub mod cubism_v3;
pub mod helpers;

pub use crate::{cubism_v1::ModelJson, cubism_v3::Model3Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Live2DModel {
    V1(ModelJson),
    V3(Model3Json),
}

impl Live2DModel {
    pub fn from_str(s: &str) -> Result<Self, serde_json::Error> {
        Ok(serde_json::from_str::<Live2DModel>(s)?)
    }
}
