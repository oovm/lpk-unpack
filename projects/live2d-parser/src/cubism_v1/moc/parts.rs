use super::*;

#[derive(Debug)]
pub struct Part {
    pub _align: [u8; 4],
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
        let align = r.read()?;
        let flag: u8 = r.read()?;
        tracing::warn!("flag: {:?}", flag);
        let locked = flag & 0x01 != 0;
        let visible = flag & 0x02 != 0;
        let name = r.read()?;
        let n = r.read()?;
        let n2 = r.read()?;
        Ok(Self { _align: align, locked, deformers: n, id: name, components: n2, visible })
    }
}
