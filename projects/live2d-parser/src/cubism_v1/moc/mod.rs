mod params;
mod parts;

use std::{ffi::CStr, str};
use serde::de::Error;
use self::{params::ParamList, parts::Part};

pub struct Moc {
    /// A memory buffer of live-2d data
    m: Vec<u8>,
    /// The version of the moc file
    version: u8,
    /// Parameter list
    params: ParamList,
    /// Parts list
    parts: Vec<Part>,
    /// Canvas width
    canvas_width: i32,
    /// Canvas height
    canvas_height: i32,
}

impl Moc {
    /// Parse moc data from a byte array
    ///
    /// ## Safety
    /// The input data must be a valid moc file
    pub unsafe fn parse(data: &[u8]) -> Result<Self, serde_json::Error> {
        // Check magic header
        if data.len() < 4 || str::from_utf8_unchecked(&data[0..3]) != "moc" {
            return Err(serde_json::Error::custom("Not a valid MOC file"));
        }

        // Read version
        let version = data[3];
        if version > 1 {
            return Err(serde_json::Error::custom("Unsupported MOC version"));
        }

        // Parse parameters and parts
        let params = ParamList::parse(data, 4)?;
        let parts = Vec::new(); // TODO: implement parts parsing

        Ok(Self {
            m: data.to_vec(),
            version,
            params,
            parts,
            canvas_width: 0,
            canvas_height: 0,
        })
    }

    /// Get the version of the moc file
    pub fn version(&self) -> u8 {
        self.version
    }

    /// Get the parameter list
    pub fn parameters(&self) -> &ParamList {
        &self.params
    }

    /// Get the parts list
    pub fn parts(&self) -> &[Part] {
        &self.parts
    }

    /// Get the canvas width
    pub fn canvas_width(&self) -> i32 {
        self.canvas_width
    }

    /// Get the canvas height
    pub fn canvas_height(&self) -> i32 {
        self.canvas_height
    }
}
