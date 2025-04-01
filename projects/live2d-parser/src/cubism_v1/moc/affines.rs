use crate::{
    cubism_v1::moc::{MocObject, MocReader},
    L2Error,
};
use tracing::{trace, warn};

#[derive(Debug)]
pub struct Affine {
    pub origin_x: f32,
    pub origin_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub reflect_x: i32,
    pub reflect_y: i32,
}

impl MocObject for Vec<Affine> {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let count = reader.read_var()?;
        let mut pivots = Vec::with_capacity(count as usize);
        trace!("Find affine: {}", count);
        for _ in 0..count {
            pivots.push(reader.read()?);
        }
        Ok(pivots)
    }
}

impl MocObject for Affine {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let origin_x = reader.read()?;
        let origin_y = reader.read()?;
        let scale_x = reader.read()?;
        let scale_y = reader.read()?;
        let rotation = reader.read()?;
        let reflect_x = if reader.version() >= 10 { reader.read()? } else { -99 };
        let reflect_y = if reader.version() >= 10 { reader.read()? } else { -99 };
        Ok(Affine { origin_x, origin_y, scale_x, scale_y, rotation, reflect_x, reflect_y })
    }
}

