use crate::{
    cubism_v1::moc::{params::Parameter, MocObject, MocReader, ObjectData},
    L2Error,
};
use integer_encoding::VarInt;
use serde::de::Error;
use tracing::debug;

#[derive(Debug)]
pub struct Part {
    pub _align: [u8; 5],
    pub flag: u8,
    pub x: ObjectData,
    /// Part name
    pub name: String,
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

impl MocObject for Vec<Part> {
    unsafe fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let mut parts = Vec::new();
        let count = r.read_var()?;
        debug!("Find parts: {}", count);
        let mut rest = data.get_unchecked(delta..);
        for _ in 0..1 {
            let out = Self::parse_one(rest)?;
            println!("{:#?}", out.0);
            parts.push(out.0);
            rest = out.1;
        }
        Ok(parts)
    }
}

impl MocObject for Part {
    unsafe fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let align = std::ptr::read(rest.as_ptr().add(0x0) as *const [u8; 5]);
        let flag = std::ptr::read(rest.as_ptr().add(0x5) as *const u8);
        let (name, rest) = read_str(rest.get_unchecked(0x6..))?;
        let (n, rest) = read_object(rest)?;
        Ok((Self { _align: align, flag, x: n, name, part_type: PartType::Normal }, &[]))
    }
}
