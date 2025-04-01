use crate::{
    cubism_v1::moc::{MocObject, MocReader},
    L2Error,
};
use tracing::{trace, warn};
use crate::cubism_v1::moc::ObjectData;

#[derive(Debug)]
pub struct PivotManager {
    pub items: Box<ObjectData>,
}

#[derive(Debug)]
pub struct Pivot {
    pub _align1: [u8; 1],
    pub id: String,
    pub values: Box<ObjectData>,
}

impl MocObject for PivotManager {
    unsafe fn read_object(reader: &MocReader) -> Result<Self, L2Error>
    where
        Self: Sized,
    {
        Ok(Self { items: Box::new(reader.read()?) })
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
        warn!("Read pivot: {}", id);
        let values = Box::new(reader.read()?);
        Ok(Self { _align1: _align, id, values })
    }
}
impl ObjectData {
    pub fn as_pivots(self) -> Vec<Pivot> {
        match self {
            ObjectData::ObjectArray(o) => o.into_iter().map(|x| x.as_pivots()).flatten().collect(),
            ObjectData::PivotManager(v) => vec![],
            _ => {
                warn!("ObjectData::as_pivots() called on non-pivot object");
                vec![]
            }
        }
    }
}
