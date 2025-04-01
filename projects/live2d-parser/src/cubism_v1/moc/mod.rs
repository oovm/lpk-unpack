mod params;
mod parts;

use self::parts::Part;
use crate::cubism_v1::moc::params::Parameter;
use integer_encoding::VarInt;
use serde::de::Error;

pub struct Moc<'i> {
    /// The version of the moc file
    version: u8,
    /// Parameter list
    params: Vec<Parameter<'i>>,
    /// Parts list
    parts: Vec<Part<'i>>,
    /// Canvas width
    canvas_width: i32,
    /// Canvas height
    canvas_height: i32,
}

#[derive(Debug)]
pub enum ObjectData {
    ObjectArray { objects: Vec<ObjectData> },
    Unknown { type_id: u64 },
}

impl<'i> Moc<'i> {
    /// Parse moc data from a byte array
    ///
    /// ## Safety
    /// The input data must be a valid moc file
    pub unsafe fn new(data: &'i [u8]) -> Result<Self, serde_json::Error> {
        // Parse parameters and parts
        let rest = data.get_unchecked(0x9..);
        let (params, rest) = Parameter::parse_many(rest)?;
        let (parts, rest) = Part::parse_many(&rest)?;
        Ok(Self { version: 0, params, parts, canvas_width: 0, canvas_height: 0 })
    }

    /// Get the version of the moc file
    pub fn version(&self) -> u8 {
        self.version
    }

    /// Get the parameter list
    pub fn parameters(&self) -> &[Parameter] {
        &[]
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

unsafe fn read_str(bytes: &[u8]) -> Result<(&str, &[u8]), serde_json::Error> {
    let (length, rest) = read_var(bytes)?;
    // tracing::trace!("String Length: {length}");
    let str = std::str::from_utf8_unchecked(rest.get_unchecked(..length));
    let rest = rest.get_unchecked(length..);
    Ok((str, rest))
}

unsafe fn read_var(bytes: &[u8]) -> Result<(usize, &[u8]), serde_json::Error> {
    match usize::decode_var(bytes) {
        Some((s, delta)) => Ok((s, bytes.get_unchecked(delta..))),
        None => Err(serde_json::Error::custom("Invalid string length"))?,
    }
}

pub struct MocReader<'i> {
    moc: &'i [u8],
    ptr: usize,
}


impl<'i> MocReader<'i> {
    pub fn new(moc: &'i [u8]) -> Self {
        Self { moc, ptr: 0 }
    }
}

unsafe fn read_object(bytes: &[u8]) -> Result<(ObjectData, &[u8]), serde_json::Error> {
    let (type_id, rest) = match u64::decode_var(bytes) {
        Some((s, delta)) => (s, bytes.get_unchecked(delta..)),
        None => Err(serde_json::Error::custom("Invalid string length"))?,
    };
    match type_id {
        15 => {
            let (objects, rest) = read_object_array(rest)?;
            Ok((ObjectData::ObjectArray { objects }, rest))
        }
        _ => Err(serde_json::Error::custom(format!("Unknown type {}", type_id))),
    }
}

unsafe fn read_object_array(bytes: &[u8]) -> Result<(Vec<ObjectData>, &[u8]), serde_json::Error> {
    match u64::decode_var(bytes) {
        Some((s, delta)) => Ok((vec![ObjectData::Unknown { type_id: s as u64 }], bytes.get_unchecked(delta..))),
        None => Err(serde_json::Error::custom("Invalid string length"))?,
    }
}
