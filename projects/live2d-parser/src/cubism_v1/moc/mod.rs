mod params;
mod parts;

use self::parts::Part;
use crate::cubism_v1::moc::params::Parameter;
use integer_encoding::VarInt;
use serde::de::Error;
use std::str;

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

enum L2ObjType {
    Unknown = -1,
    Null = 0,
    String = 1,
    Color = 10,
    RectD = 11,
    RectF = 12,
    PointD = 13,
    PointF = 14,
    ObjectArray = 15,
    IntArray = 16,
    IntArray2 = 25,
    Matrix2x3 = 17,
    Rect = 21,
    Point = 22,
    Array = 23,
    DoubleArray = 26,
    FloatArray = 27,
    /// Object Reference
    ObjectReference = 33,
    DrawDataID = 50,
    BaseDataID = 51,
    ParamID = 60,
    PartsDataID = 134,
    ParamDefF = 131,
    PartsData = 133,
    ModelImpl = 136,
    ParamDefList = 137,
    AvatarPartsItem = 142,
    DDTexture = 70,
    Affine = 69,
    RotationDeformer = 68,
    ParamPivots = 67,
    PivotManager = 66,
    CurvedSurfaceDeformer = 65,
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
    let (length, delta) = match u64::decode_var(bytes) {
        Some(s) => s,
        None => Err(serde_json::Error::custom("Invalid string length"))?,
    };
    let end = delta + length as usize;
    let str = std::str::from_utf8_unchecked(bytes.get_unchecked(delta..end));
    let rest = bytes.get_unchecked(end..);
    Ok((str, rest))
}
