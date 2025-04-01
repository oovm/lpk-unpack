use super::*;
use crate::cubism_v1::moc::pivots::Pivot;

pub enum DeformerType {
    Dummy = 0,
    Rotation = 1,
    CurvedSurface = 2,
}

#[derive(Debug)]
pub struct RotationDeformer {
    id: String,
    target_id: String,
    pivots: Vec<Pivot>,
}

impl MocObject for RotationDeformer {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let id = reader.read()?;
        println!("{}", id);
        let target_id = reader.read()?;
        println!("{}", target_id);
        let pivots = reader.read()?;
        Ok(Self { id, target_id, pivots })
    }
}
