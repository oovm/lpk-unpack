use super::*;
use crate::cubism_v1::moc::{affines::Affine, pivots::Pivot};

pub enum DeformerType {
    Dummy = 0,
    Rotation = 1,
    CurvedSurface = 2,
}

#[derive(Debug)]
pub struct RotationDeformer {
    pub id: String,
    pub target_id: String,
    pub pivots: Vec<Pivot>,
    pub pivots_opacity: Vec<f32>,
    pub affine: Vec<Affine>,
}
#[derive(Debug)]
pub struct CurvedSurfaceDeformer {
    pub id: String,
    pub target_id: String,
    pub row: i32,
    pub column: i32,
    pub pivots: Vec<Pivot>,
    pub opacities: Vec<f32>,
}

impl MocObject for RotationDeformer {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let id = reader.read()?;
        let target_id = reader.read()?;
        // let x = reader.read_var()?;
        // tracing::warn!("id={}", x);
        let pivots: ObjectData = reader.read()?;
        let affine: ObjectData = reader.read()?;
        let opacities = if reader.version() >= 10 { reader.read()? } else { Vec::new() };
        Ok(Self {
            id,
            target_id,
            //
            pivots: pivots.as_pivots(),
            //
            affine: affine.as_affine(),
            pivots_opacity: opacities,
        })
    }
}

impl MocObject for CurvedSurfaceDeformer {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let id = reader.read()?;
        let target_id = reader.read()?;

        let row = reader.read()?;
        let column = reader.read()?;

        let pivots: ObjectData = reader.read()?;
        let opacities = if reader.version() >= 10 { reader.read()? } else { Vec::new() };
        Ok(Self {
            id,
            target_id,
            row,
            column,
            //
            pivots: pivots.as_pivots(),
            opacities,
        })
    }
}
