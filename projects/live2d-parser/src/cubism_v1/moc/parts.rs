use super::*;
use tracing::warn;

#[derive(Debug)]
pub struct Part {
    /// Part name
    pub id: String,
    pub locked: bool,
    pub visible: bool,
    pub deformers: ObjectData,
    pub components: ObjectData,
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
        let flag: u8 = r.read()?;
        warn!("flag: {:?}", flag);
        let locked = flag & 0x01 != 0;
        let visible = flag & 0x02 != 0;
        let name = r.read()?;
        warn!("name: {:?}", name);
        let deformers = r.read()?;
        let components = r.read()?;
        Ok(Self { locked, deformers, id: name, components, visible })
    }
}

impl ObjectData {
    pub fn as_parts(&self) -> Vec<Part> {
        match self {
            s => {
                warn!("ObjectData::as_parts() called on non-pivot object {s:?}");
                vec![]
            }
        }
    }
}
