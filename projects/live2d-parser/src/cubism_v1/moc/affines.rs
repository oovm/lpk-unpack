use crate::{
    cubism_v1::moc::{MocObject, MocReader},
    L2Error,
};
use tracing::{trace, warn};

#[derive(Debug)]
pub struct Affine {
    pub _align1: [u8; 2],
    pub id: String,
    pub values: Vec<f32>,
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
        let _align = reader.read()?;
        let id = reader.read()?;
        warn!("Read affine: {}", id);
        let values = reader.read()?;
        Ok(Self { _align1: _align, id, values })
    }
}
