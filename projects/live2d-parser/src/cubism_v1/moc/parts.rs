use super::*;

#[derive(Debug)]
pub struct Part {
    pub _align: [u8; 5],
    pub flag: u8,
    /// Part name
    pub name: String,
    /// Part type
    pub part_type: PartType,
    pub unknown1: ObjectData,
    pub unknown2: ObjectData,
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
        for _ in 0..count {
            let out = r.read()?;
            println!("{:#?}", out);
            parts.push(out);
        }
        Ok(parts)
    }
}

impl MocObject for Part {
    unsafe fn read_object(r: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let align = r.read()?;
        let flag = r.read()?;
        let name = r.read()?;
        let n = r.read()?;
        let n2 = r.read()?;
        Ok(Self { _align: align, flag, unknown1: n, name, part_type: PartType::Normal, unknown2: n2 })
    }
}
