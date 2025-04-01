use crate::cubism_v1::moc::{params::Parameter, read_str, read_object, read_var, ObjectData};
use integer_encoding::VarInt;
use serde::de::Error;
use tracing::debug;

#[derive(Debug)]
pub struct Part<'i> {
    pub _align: [u8; 5],
    pub flag: u8,
    pub x: ObjectData,
    /// Part name
    pub name: &'i str,
    /// Part type
    pub part_type: PartType,
}

#[derive(Debug)]
pub enum PartType {
    /// Normal part
    Normal,
    /// Mesh part
    Mesh,
}

impl<'i> Part<'i> {
    pub unsafe fn parse_many(data: &'i [u8]) -> Result<(Vec<Part<'i>>, &'i [u8]), serde_json::Error> {
        let mut parts = Vec::new();
        let (count, delta) = match u64::decode_var(data) {
            Some(s) => s,
            None => Err(serde_json::Error::custom("Invalid string length"))?,
        };
        debug!("Find parts: {}", count);
        let mut rest = data.get_unchecked(delta..);
        for _ in 0..1 {
            let out = Self::parse_one(rest)?;
            println!("{:#?}", out.0);
            parts.push(out.0);
            rest = out.1;
        }
        Ok((parts, rest))
    }

    /// Parse part from moc data
    ///
    /// ## Safety
    /// The input data must be a valid moc file
    pub unsafe fn parse_one(rest: &'i [u8]) -> Result<(Part<'i>, &'i [u8]), serde_json::Error> {
        let align = std::ptr::read(rest.as_ptr().add(0x0) as *const [u8; 5]);
        let flag = std::ptr::read(rest.as_ptr().add(0x5) as *const u8);
        let (name, rest) = read_str(rest.get_unchecked(0x6..))?;
        let (n, rest) = read_object(rest)?;
        Ok((Self { _align: align, flag, x: n, name, part_type: PartType::Normal }, &[]))
    }
}
