use super::*;
use crate::cubism_v1::moc::{affines::Affine, pivots::Pivot};

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
    affine: Vec<Affine>,
    opacities: Vec<f32>,
}
#[derive(Debug)]
pub struct CurvedSurfaceDeformer {
    _align1: [u8; 1],
    id: String,
    _align2: [u8; 1],
    target_id: String,
    row: i32,
    column: i32,
    pivots: Vec<Pivot>,
    affine: Vec<Affine>,
    opacities: Vec<f32>,
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
        let pivots: ObjectData = reader.read()?;
        let affine = reader.read()?;
        let opacities = if (reader.version() >= 10) { reader.read()? } else { Vec::new() };
        Ok(Self { _align1, _align2, id, target_id, pivots: pivots.as_pivots(), affine, opacities })
    }
}

impl MocObject for CurvedSurfaceDeformer {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let _align1 = reader.read()?;
        let id = reader.read()?;
        let _align2 = reader.read()?;
        let target_id = reader.read()?;

        let row = reader.read()?;
        let column = reader.read()?;

        let pivots: ObjectData = reader.read()?;
        let affine = reader.read()?;
        let opacities = if (reader.version() >= 10) { reader.read()? } else { Vec::new() };
        Ok(Self { _align1, _align2, id, target_id, row, column, pivots: pivots.as_pivots(), affine, opacities })
    }
}
