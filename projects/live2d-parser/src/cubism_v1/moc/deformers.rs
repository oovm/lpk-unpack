use super::*;
use crate::cubism_v1::moc::pivots::Pivot;

pub enum DeformerType {
    Dummy = 0,
    Rotation = 1,
    CurvedSurface = 2,
}


#[derive(Debug)]
pub struct RotationDeformer {
    _align1: [u8; 1],
    id: String,
    _align2: [u8; 1],
    target_id: String,
    pivots: Vec<Pivot>,
}

impl MocObject for RotationDeformer {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let _align1 = reader.read()?;
        let id = reader.read()?;
        let _align2 = reader.read()?;
        let target_id = reader.read()?;
        let o: ObjectData = reader.read()?;
        panic!("{:#?}", o);
        Ok(Self { _align1, _align2, id, target_id, pivots: vec![] })
    }
}
