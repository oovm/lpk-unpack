use super::*;

pub enum DeformerType {
    Dummy = 0,
    Rotation = 1,
    CurvedSurface = 2,
}

#[derive(Debug)]
pub struct RotationDeformer {
    id: String,
    target_id: String,
}

impl MocObject for RotationDeformer {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        Ok(Self { id: reader.read()?, target_id: reader.read()? })
    }
}
