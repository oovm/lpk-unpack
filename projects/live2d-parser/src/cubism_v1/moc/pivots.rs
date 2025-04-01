use crate::{
    cubism_v1::moc::{MocObject, MocReader, ObjectData},
    L2Error,
};
use integer_encoding::VarInt;
use tracing::trace;

#[derive(Debug)]
pub struct PivotManager {
    items: Vec<Pivot>,
}

#[derive(Debug)]
pub struct Pivot {
    _align: [u8; 2],
    id: String,
    values: Vec<f32>,
}

impl MocObject for PivotManager {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        Ok(Self { items: reader.read()? })
    }
}

impl MocObject for Vec<Pivot> {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let count = reader.read_var()?;
        let mut pivots = Vec::with_capacity(count);
        trace!("Find pivots: {}", count);
        for _ in 0..count {
            pivots.push(reader.read()?);
        }
        Ok(pivots)
    }
}

impl MocObject for Pivot {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        let _align = reader.read()?;
        let id = reader.read()?;
        let values = reader.read()?;
        Ok(Self { _align, id, values })
    }
}
